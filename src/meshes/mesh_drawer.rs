use glium::{Display, DrawParameters, Frame, Program, Surface, uniform};
use glium::glutin::surface::WindowSurface;
use nalgebra::Matrix4;
use crate::meshes::mesh::Mesh;

pub struct MeshDrawer {
    program: Program,
    drawing_parameters: DrawParameters<'static>,
}

impl MeshDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 140
    
            in vec3 position;
            
            uniform mat4 perspective;
            uniform mat4 model;
            uniform mat4 view;
    
            void main() {
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
    
            out vec4 color;
            
            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        let program = Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let mut drawing_parameters = DrawParameters::default();
        drawing_parameters.depth = glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        };

        Self {
            program,
            drawing_parameters,
        }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        mesh: &Mesh,
        perspective: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
    ) {
        
        target
            .draw(
                &mesh.vertex_buffer,
                &mesh.index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data.0,
                    model: model.data.0,
                    view: view.data.0,
                },
                &self.drawing_parameters,
            )
            .unwrap();
    }
}
