mod meshes;
mod cubes;

use std::fs::File;
use std::io::BufReader;
use egui::Shape::Mesh;
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use image::io::Reader;
use nalgebra::{Matrix4, Point3, Vector3};
use winit::{event, event_loop};
use winit::event::WindowEvent;
use crate::cubes::cube::Cube;
use crate::cubes::cube_drawer::CubeDrawer;
use crate::meshes::mesh_drawer::MeshDrawer;
use crate::meshes::read_mesh::read_mesh;

fn main() {
    let mut width = 800;
    let mut height = 600;

    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Duck")
        .with_inner_size(width, height)
        .build(&event_loop);

    let mut egui_glium = egui_glium::EguiGlium::new(&display, &window, &event_loop);
    
    let duck_mesh = read_mesh("meshes/duck.txt", &display);
    let mesh_drawer = MeshDrawer::new(&display);

    let duck_texture = read_duck_texture(&display);
    let vulkan_texture = read_vulkan_texture(&display);
    let sky_texture = read_sky_texture(&display);
    let sand_texture = read_sand_texture(&display);
    
    let model = Matrix4::new_translation(&Vector3::new(0.0, -1.0, 0.0)) * Matrix4::new_scaling(0.01);
    let mut perspective = Matrix4::new_perspective(width as f32 / height as f32, std::f32::consts::PI / 2.0, 0.1, 100.0);
    let view = Matrix4::look_at_rh(
        &Point3::new(0.0, 0.0, 5.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vector3::y(),
    );
    
    let cube = Cube::new(&display);
    let cube_drawer = CubeDrawer::new(&display);

    event_loop.run(move |event, _window_target, control_flow| {
        let mut redraw = || {
            let repaint_after = egui_glium.run(&window, |egui_ctx| {});

            *control_flow = if repaint_after.is_zero() {
                window.request_redraw();
                event_loop::ControlFlow::Poll
            } else if let Some(repaint_after_instant) =
                std::time::Instant::now().checked_add(repaint_after)
            {
                event_loop::ControlFlow::WaitUntil(repaint_after_instant)
            } else {
                event_loop::ControlFlow::Wait
            };

            let mut target = display.draw();

            target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            
            mesh_drawer.draw(&mut target, &duck_mesh, &perspective, &view, &model, &duck_texture);
            cube_drawer.draw(&mut target, &cube, &perspective, &view, &Matrix4::identity(), &vulkan_texture, &sky_texture, &sand_texture);
            
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
