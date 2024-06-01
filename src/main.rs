mod meshes;
mod cubes;
mod water;

use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::vec;
use chrono::Local;
use egui::{ScrollArea, Slider, Widget};
use egui::Shape::Mesh;
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use image::io::Reader;
use nalgebra::{clamp, Matrix4, Point2, Point3, Rotation3, Unit, Vector2, Vector3, Vector4};
use rand::prelude::ThreadRng;
use rand::{Rng, RngCore, thread_rng};
use winit::{event, event_loop};
use winit::event::{MouseButton, WindowEvent};
use winit::event::ElementState::Pressed;
use crate::cubes::cube::Cube;
use crate::cubes::cube_drawer::CubeDrawer;
use crate::meshes::mesh_drawer::MeshDrawer;
use crate::meshes::read_mesh::read_mesh;
use crate::water::water::Water;
use crate::water::water_drawer::WaterDrawer;
use crate::water::water_normal_computer::WaterNormalComputer;

fn main() {
    let mut width = 800;
    let mut height = 600;

    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Duck")
        .with_inner_size(width, height)
        .build(&event_loop);

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &window, &event_loop);

    let mut rng = thread_rng();
    
    let duck_mesh = read_mesh("meshes/duck.txt", &display);
    let mesh_drawer = MeshDrawer::new(&display);
    let mut duck_position = Point2::new(0.0, 0.0f32);
    let mut b_spline = get_b_spline(duck_position, &mut rng);
    let mut b_spline_t = 0.0f32;
    let mut duck_speed = 0.01f32;

    let duck_texture = read_duck_texture(&display);
    let vulkan_texture = read_vulkan_texture(&display);
    let sky_texture = read_sky_texture(&display);
    let sand_texture = read_sand_texture(&display);
    
    let mut perspective = Matrix4::new_perspective(width as f32 / height as f32, std::f32::consts::PI / 2.0, 0.1, 100.0);
    
    let cube = Cube::new(&display);
    let cube_drawer = CubeDrawer::new(&display);
    
    let water = Water::new(&display);
    let water_drawer = WaterDrawer::new(&display);
    let mut water_height = 0f32;
    let water_normal_computer = WaterNormalComputer::new(&display);
    let mut time_to_compute = 0.0f32;

    let mut mouse_position = (0.0, 0.0);
    let mut camera_direction = Vector3::new(0.0f32, 0.0, 1.0);
    let mut camera_angle = Vector3::new(0.0f32, 0.0, 0.0);
    let mut camera_up = Vector3::new(0.0f32, 1.0, 0.0);
    let mut camera_distant = 4.0f32;
    let mut view = Matrix4::look_at_rh(
        &Point3::from_slice((-camera_distant * camera_direction).as_slice()),
        &Point3::new(0.0, 0.0, 0.0),
        &camera_up,
    );
    let mut mouse_middle_button_pressed = false;
    
    let mut previous_time = Local::now();

    event_loop.run(move |event, _window_target, control_flow| {
        let mut redraw = || {
            let current_time = Local::now();
            let duration = current_time - previous_time;
            let duration_in_seconds = duration.num_microseconds().unwrap_or(1) as f64 / 1_000_000.0;
            let fps = 1.0 / duration_in_seconds;
            previous_time = current_time;
            
            egui_glium.run(&window, |egui_ctx| {
                egui::Window::new("panel").show(egui_ctx, |ui| {
                    Slider::new(&mut water_height, -0.9..=0.9)
                        .step_by(0.05)
                        .text("water height")
                        .ui(ui);

                    Slider::new(&mut duck_speed, -0.001..=0.1)
                        .step_by(0.001)
                        .text("Duck speed")
                        .ui(ui);

                    ui.label(format!("Duck position: ({:.1}, {:.1})", duck_position.x, duck_position.y));

                    ui.label(format!("P[0]: ({:.1}, {:.1})", b_spline[0].x, b_spline[0].y));
                    ui.label(format!("P[1]: ({:.1}, {:.1})", b_spline[1].x, b_spline[1].y));
                    ui.label(format!("P[2]: ({:.1}, {:.1})", b_spline[2].x, b_spline[2].y));
                    ui.label(format!("P[3]: ({:.1}, {:.1})", b_spline[3].x, b_spline[3].y));
                    
                    ui.label(format!("FPS: {:.1}", fps));
                });
            });
            
            *control_flow = event_loop::ControlFlow::Poll;

            let mut target = display.draw();

            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

            time_to_compute += duration_in_seconds as f32;
            if time_to_compute >= water_normal_computer.get_dt()
            {
                let x = rng.next_u32() % 256 * 12;
                let y = rng.next_u32() % 256 * 12;

                if x < 256 && y < 256 { 
                    water_normal_computer.bend(x as i32, y as i32);
                }
                
                b_spline_t += duck_speed;
                
                if b_spline_t >= 1.0
                {
                    b_spline_t = 0.0;
                    b_spline = [b_spline[1], b_spline[2], b_spline[3], get_random_point(b_spline[3], &mut rng)];
                }
                duck_position = get_b_spline_value(b_spline, b_spline_t);

                water_normal_computer.bend((duck_position.x * 128.0 / 5.0 + 128.0) as i32, (duck_position.y * 128.0 / 5.0 + 128.0) as i32);

                water_normal_computer.compute();
                
                time_to_compute -= water_normal_computer.get_dt();
            }
            
            mesh_drawer.draw(&mut target, &duck_mesh, &perspective, &view, &(Matrix4::new_translation(&Vector3::new(duck_position.x, -0.2 + water_height * 5.0, duck_position.y)) * get_rotation(get_b_spline_derivative_value(b_spline, b_spline_t)) * Matrix4::new_scaling(0.01)), &duck_texture);
            cube_drawer.draw(&mut target, &cube, &perspective, &view, &Matrix4::new_scaling(5.0), &vulkan_texture, &sky_texture, &sand_texture);
            water_drawer.draw(&mut target, &water, &perspective, &view, &Matrix4::new_scaling(5.0), &Point3::from_slice((-camera_distant * camera_direction).as_slice()), water_height, &vulkan_texture, &sky_texture, &sand_texture, &water_normal_computer.normal_tex);

            egui_glium.paint(&display, &mut target);

            target.finish().unwrap();
        };
        match event {
            event::Event::RedrawRequested(_) => redraw(),

            event::Event::WindowEvent { event, .. } => {
                use event::WindowEvent;
                match &event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                        *control_flow = event_loop::ControlFlow::Exit;
                    }
                    WindowEvent::Resized(new_size) => {
                        display.resize((*new_size).into());
                        width = new_size.width;
                        height = new_size.height;
                        perspective = Matrix4::new_perspective(width as f32 / height as f32, std::f32::consts::PI / 2.0, 0.1, 100.0);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let delta = (position.x - mouse_position.0, position.y - mouse_position.1);
                        mouse_position = (position.x, position.y);
                        if mouse_middle_button_pressed {
                            camera_angle.x += delta.1 as f32 * 0.01;
                            camera_angle.y += delta.0 as f32 * 0.01 * if camera_angle.x.cos() < 0.0 { -1.0 } else { 1.0 };
                            camera_direction = (Matrix4::from_euler_angles(camera_angle.x, camera_angle.y, 0.0) * Vector4::new(0.0, 0.0, 1.0, 0.0)).xyz();
                            camera_up = (Matrix4::from_euler_angles(camera_angle.x, camera_angle.y, 0.0) * Vector4::new(0.0, 1.0, 0.0, 0.0)).xyz();
                            view = Matrix4::look_at_rh(
                                &Point3::from_slice((-camera_distant * camera_direction).as_slice()),
                                &Point3::new(0.0, 0.0, 0.0),
                                &camera_up,
                            );
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if *button == MouseButton::Middle {
                            mouse_middle_button_pressed = *state == Pressed;
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        match delta {
                            event::MouseScrollDelta::LineDelta(_x, y) => {
                                camera_distant += -y * 0.1;
                                view = Matrix4::look_at_rh(
                                    &Point3::from_slice((-camera_distant * camera_direction).as_slice()),
                                    &Point3::new(0.0, 0.0, 0.0),
                                    &camera_up,
                                );
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                let event_response = egui_glium.on_event(&event);

                if event_response.repaint {
                    window.request_redraw();
                }
            }
            event::Event::NewEvents(event::StartCause::ResumeTimeReached { .. }) => {
                window.request_redraw();
            }
            event::Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

fn read_duck_texture(display: &Display<WindowSurface>) -> glium::texture::Texture2d {
    let image = image::load(std::io::Cursor::new(&include_bytes!("../textures/ducktex.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, image).unwrap()
}

fn read_vulkan_texture(display: &Display<WindowSurface>) -> glium::texture::Texture2d {
    let image = Reader::open("textures/vulkan.jpg").unwrap().decode().unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, image).unwrap()
}

fn read_sky_texture(display: &Display<WindowSurface>) -> glium::texture::Texture2d {
    let image = Reader::open("textures/sky.jpg").unwrap().decode().unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, image).unwrap()
}

fn read_sand_texture(display: &Display<WindowSurface>) -> glium::texture::Texture2d {
    let image = Reader::open("textures/sand.jpg").unwrap().decode().unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    glium::texture::Texture2d::new(display, image).unwrap()
}

fn get_b_spline(p0: Point2<f32>, rng: &mut ThreadRng) -> [Point2<f32>; 4] {
    let p1 = get_random_point(p0, rng);
    let p2 = get_random_point(p1, rng);
    let p3 = get_random_point(p2, rng);
    [p0, p1, p2, p3]
}

fn get_random_point(p0: Point2<f32>, rng: &mut ThreadRng) -> Point2<f32> {
    let direction = Vector2::new(rng.gen_range(-100..100) as f32, rng.gen_range(-100..100) as f32).normalize();
    Point2::new(clamp(p0.x + direction.x * 3.0, -3.0, 3.0), clamp(p0.y + direction.y * 3.0, -3.0, 3.0))
}

fn get_b_spline_value(b_spline: [Point2<f32>; 4], t: f32) -> Point2<f32> {
    let t2 = t * t;
    let t3 = t2 * t;
    let b0 = (-t3 + 3.0 * t2 - 3.0 * t + 1.0) / 6.0;
    let b1 = (3.0 * t3 - 6.0 * t2 + 4.0) / 6.0;
    let b2 = (-3.0 * t3 + 3.0 * t2 + 3.0 * t + 1.0) / 6.0;
    let b3 = t3 / 6.0;
    Point2::new(
        b_spline[0].x * b0 + b_spline[1].x * b1 + b_spline[2].x * b2 + b_spline[3].x * b3,
        b_spline[0].y * b0 + b_spline[1].y * b1 + b_spline[2].y * b2 + b_spline[3].y * b3)
}

fn get_b_spline_derivative_value(b_spline: [Point2<f32>; 4], t: f32) -> Vector2<f32> {
    const EPS: f32 = 0.0001;
    let t1 = t - EPS;
    let t2 = t + EPS;
    let p1 = get_b_spline_value(b_spline, t1);
    let p2 = get_b_spline_value(b_spline, t2);
    (p2.coords - p1.coords) / (2.0 * EPS)
}

fn get_rotation(direction: Vector2<f32>) -> Matrix4<f32> {
    let c = (direction.x * direction.x + direction.y * direction.y).sqrt();
    let cos = -direction.x / c;
    let sin = direction.y / c;
    Matrix4::new(
        cos, 0.0, sin, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin, 0.0, cos, 0.0,
        0.0, 0.0, 0.0, 1.0)
}
