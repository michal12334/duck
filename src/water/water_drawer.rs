use glium::{Display, DrawParameters, Frame, Program, Surface, Texture2d, uniform};
use glium::glutin::surface::WindowSurface;
use nalgebra::Matrix4;
use crate::water::water::Water;

pub struct WaterDrawer {
    program: Program,
    drawing_parameters: DrawParameters<'static>,
}

impl WaterDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 140
    
            in vec3 position;
            
            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;
    
            void main() {
                gl_Position = perspective * view * model * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 140
            
            out vec4 color;

            uniform sampler2D tex1;
            uniform sampler2D tex2;
            uniform sampler2D tex3;
            
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
        drawing_parameters.backface_culling = glium::draw_parameters::BackfaceCullingMode::CullingDisabled;

        Self {
            program,
            drawing_parameters,
        }
    }

    pub fn draw(
        &self,
        target: &mut Frame,
        water: &Water,
        perspective: &Matrix4<f32>,
        view: &Matrix4<f32>,
        model: &Matrix4<f32>,
        texture1: &Texture2d,
        texture2: &Texture2d,
        texture3: &Texture2d,
    ) {
        target
            .draw(
                &water.vertex_buffer,
                &water.index_buffer,
                &self.program,
                &uniform! {
                    perspective: perspective.data.0,
                    view: view.data.0,
                    model: model.data.0,
                    tex1: texture1,
                    tex2: texture2,
                    tex3: texture3,
                },
                &self.drawing_parameters,
            )
            .unwrap();
    }
}
