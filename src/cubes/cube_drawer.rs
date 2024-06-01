use glium::glutin::surface::WindowSurface;
use glium::{uniform, Display, DrawParameters, Frame, Program, Surface, Texture2d};
use nalgebra::Matrix4;

use crate::cubes::cube::Cube;

pub struct CubeDrawer {
    program: Program,
    drawing_parameters: DrawParameters<'static>,
}

impl CubeDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 140
    
            in vec3 position;
            in vec2 tex_coords;
            in int tex_index;
            out vec2 v_tex_coords;
            flat out int v_tex_index;

            uniform mat4 perspective;
            uniform mat4 model;
            uniform mat4 view;
    
            void main() {
                v_tex_coords = tex_coords;
                v_tex_index = tex_index;
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
    
            in vec2 v_tex_coords;
            flat in int v_tex_index;
            out vec4 color;
            
            uniform sampler2D tex1;
            uniform sampler2D tex2;
            uniform sampler2D tex3;
            
            void main() {
                if (v_tex_index == 0) {
                    color = texture(tex1, v_tex_coords);
                } else if (v_tex_index == 1) {
                    color = texture(tex2, v_tex_coords);
                } else {
                    color = texture(tex3, v_tex_coords);
                }
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
        mesh: &Cube,
        perspective: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
        texture1: &Texture2d,
        texture2: &Texture2d,
        texture3: &Texture2d,
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
                    tex1: texture1,
                    tex2: texture2,
                    tex3: texture3,
                },
                &self.drawing_parameters,
            )
            .unwrap();
    }
}
