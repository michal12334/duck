use glium::glutin::surface::WindowSurface;
use glium::{uniform, Display, DrawParameters, Frame, Program, Surface, Texture2d};
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
            in vec2 tex_coords;
            out vec2 v_tex_coords;
            
            uniform mat4 perspective;
            uniform mat4 model;
            uniform mat4 view;
    
            void main() {
                v_tex_coords = tex_coords;
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
    
            in vec2 v_tex_coords;
            out vec4 color;
            
            uniform sampler2D tex;
            
            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;

        let program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        let mut drawing_parameters = DrawParameters::default();
        drawing_parameters.depth = glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        };
        drawing_parameters.backface_culling =
            glium::draw_parameters::BackfaceCullingMode::CullClockwise;

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
        texture: &Texture2d,
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
                    tex: texture,
                },
                &self.drawing_parameters,
            )
            .unwrap();
    }
}
