use glium::{Display, DrawParameters, Frame, Program, Surface, Texture2d, uniform};
use glium::glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Point3, Vector3};
use crate::water::water::Water;

pub struct WaterDrawer {
    program: Program,
    drawing_parameters: DrawParameters<'static>,
}

impl WaterDrawer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let vertex_shader_src = r#"
            #version 460 core
    
            in vec3 position;
            in vec2 tex_coords;
            
            out vec3 local_position;
            out vec3 world_position;
            out vec2 v_tex_coords;
            
            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;
            uniform float height;
    
            void main() {
                v_tex_coords = tex_coords;
                local_position = position;
                local_position.y = height;
                vec4 world_position4 = model * vec4(local_position, 1.0);
                world_position = world_position4.xyz;
                gl_Position = perspective * view * world_position4;
            }
        "#;

        let fragment_shader_src = r#"
            #version 460 core
            
            in vec3 local_position;
            in vec3 world_position;
            in vec2 v_tex_coords;
            
            out vec4 color;

            uniform sampler2D tex1;
            uniform sampler2D tex2;
            uniform sampler2D tex3;
            uniform vec3 camera_position;
            uniform sampler2D normal_tex;
            
            vec3 intersect_ray(vec3 p, vec3 v) {
                vec3 t_m = (vec3(-1, -1, -1) - p) / v;
                vec3 t_p = (vec3(1, 1, 1) - p) / v;
                vec3 t = max(t_m, t_p);
                float a = min(t.x, min(t.y, t.z));
                return p + a * v;
            }

            float fresnel(vec3 v, vec3 n) {
                float co = max(dot(v, n), 0);
                float F0 = 0.14;
                return F0 + (1 - F0) * pow(1 - co, 5);
            }
            
            vec4 get_color(vec3 p, vec3 v) {
                vec3 r = intersect_ray(p, v);
                float ax = abs(r.x);
                float ay = abs(r.y);
                float az = abs(r.z);
                if (ax > ay && ax > az) {
                    if (r.x > 0) {
                        float tx = r.z / 2.0 + 0.5;
                        float ty = -r.y / 2.0 + 0.5;
                        return texture(tex1, vec2(tx, ty));
                    } else {
                        float tx = r.z / 2.0 + 0.5;
                        float ty = -r.y / 2.0 + 0.5;
                        return texture(tex1, vec2(tx, ty));
                    }
                } else if (ay > ax && ay > az) {
                    if (r.y > 0) {
                        float tx = r.x / 2.0 + 0.5;
                        float ty = -r.z / 2.0 + 0.5;
                        return texture(tex2, vec2(tx, ty));
                    } else {
                        float tx = r.x / 2.0 + 0.5;
                        float ty = -r.z / 2.0 + 0.5;
                        return texture(tex3, vec2(tx, ty));
                    }
                } else {
                    if (r.z > 0) {
                        float tx = r.x / 2.0 + 0.5;
                        float ty = -r.y / 2.0 + 0.5;
                        return texture(tex1, vec2(tx, ty));
                    } else {
                        float tx = r.x / 2.0 + 0.5;
                        float ty = -r.y / 2.0 + 0.5;
                        return texture(tex1, vec2(tx, ty));
                    }
                }
                return vec4(0, 1, 0, 1);
            }
            
            void main() {
                vec3 view_vector = normalize(camera_position - world_position);
                vec3 normal = normalize(texture(normal_tex, v_tex_coords).xyz);
                float n1n2 = 3.0/4.0;
                
                bool below = dot(view_vector, normal) < 0;
                if (below) {
                    normal.y = -normal.y;
                    n1n2 = 1.0 / n1n2;
                }

                vec3 reflected = reflect(-view_vector, normal);
                vec3 refracted = refract(-view_vector, normal, n1n2);
                float fresnel_value = fresnel(view_vector, normal);
                
                vec4 reflected_color = get_color(local_position, reflected);
                vec4 refracted_color = get_color(local_position, refracted);
                
                if (!all(isnan(refracted))) {
                    color = mix(refracted_color, reflected_color, fresnel_value);
                } else {
                    color = reflected_color;
                }
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
        camera_position: &Point3<f32>,
        water_height: f32,
        texture1: &Texture2d,
        texture2: &Texture2d,
        texture3: &Texture2d,
        normal_tex: &Texture2d,
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
                    camera_position: [camera_position.x, camera_position.y, camera_position.z],
                    height: water_height,
                    tex1: texture1,
                    tex2: texture2,
                    tex3: texture3,
                    normal_tex: normal_tex,
                },
                &self.drawing_parameters,
            )
            .unwrap();
    }
}
