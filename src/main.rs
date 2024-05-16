mod meshes;

use egui::Shape::Mesh;
use glium::Surface;
use nalgebra::Matrix4;
use winit::{event, event_loop};
use winit::event::WindowEvent;
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

    let image = image::load(std::io::Cursor::new(&include_bytes!("../textures/ducktex.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    
    let model = Matrix4::new_translation(&nalgebra::Vector3::new(0.0, -1.0, 0.0)) * Matrix4::new_scaling(0.01);
    let perspective = Matrix4::new_perspective(width as f32 / height as f32, std::f32::consts::PI / 2.0, 0.1, 100.0);
    let view = Matrix4::look_at_rh(
        &nalgebra::Point3::new(0.0, 0.0, 5.0),
        &nalgebra::Point3::new(0.0, 0.0, 0.0),
        &nalgebra::Vector3::y(),
    );

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
            
            mesh_drawer.draw(&mut target, &duck_mesh, &perspective, &view, &model, &texture);
            
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
