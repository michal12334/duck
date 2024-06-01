use glium::glutin::surface::WindowSurface;
use glium::program::ComputeShader;
use glium::uniforms::{ImageUnitAccess, ImageUnitFormat};
use glium::{uniform, Display, Texture2d};

pub struct WaterNormalComputer {
    height_compute_shader: ComputeShader,
    swap_compute_shader: ComputeShader,
    normal_compute_shader: ComputeShader,
    bend_compute_shader: ComputeShader,
    tex1: Texture2d,
    tex2: Texture2d,
    pub normal_tex: Texture2d,
    a: f32,
    b: f32,
    dt: f32,
}

impl WaterNormalComputer {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let tex1 = Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            256,
            256,
        )
        .unwrap();

        let tex2 = Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::F32,
            glium::texture::MipmapsOption::NoMipmap,
            256,
            256,
        )
        .unwrap();

        let normal_tex = Texture2d::empty_with_format(
            display,
            glium::texture::UncompressedFloatFormat::U8U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            256,
            256,
        )
        .unwrap();

        let height_compute_shader = ComputeShader::from_source(
            display,
            r#"\
            #version 460 core
            layout(local_size_x = 8, local_size_y = 4, local_size_z = 1) in;

            layout(r32f) readonly uniform image2D tex1;
            layout(r32f) uniform image2D tex2;

            uniform float A;
            uniform float B;

            float get_d(vec2 i) {
                float lx = max(i.x / 128.0f, 2.0f - i.x / 128.0f);
                float ly = max(i.y / 128.0f, 2.0f - i.y / 128.0f);
                float l = max(lx, ly);
                return 0.95 * min(1.0, l / 0.2);
            }

            void main() {
                ivec2 i = ivec2(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);
                float z1 = imageLoad(tex1, ivec2(i.x, i.y + 1)).x;
                float z2 = imageLoad(tex1, ivec2(i.x, i.y - 1)).x;
                float z3 = imageLoad(tex1, ivec2(i.x + 1, i.y)).x;
                float z4 = imageLoad(tex1, ivec2(i.x - 1, i.y)).x;
                float z5 = imageLoad(tex1, ivec2(i.x, i.y)).x;
                float z6 = imageLoad(tex2, ivec2(i.x, i.y)).x;

                float d = get_d(i);

                float c = d * (A * (z1 + z2 + z3 + z4) + B * z5 - z6);
                imageStore(tex2, i, vec4(c, 0, 0, 0));
            }
            "#,
        )
        .unwrap();

        let swap_compute_shader = ComputeShader::from_source(
            display,
            r#"\
            #version 460 core
            layout(local_size_x = 8, local_size_y = 4, local_size_z = 1) in;

            layout(r32f) uniform image2D tex1;
            layout(r32f) uniform image2D tex2;

            void main() {
                ivec2 i = ivec2(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);
                vec4 c1 = imageLoad(tex1, i);
                vec4 c2 = imageLoad(tex2, i);
                imageStore(tex1, i, c2);
                imageStore(tex2, i, c1);
            }
            "#,
        )
        .unwrap();

        let normal_compute_shader = ComputeShader::from_source(
            display,
            r#"\
            #version 460 core
            layout(local_size_x = 8, local_size_y = 4, local_size_z = 1) in;

            layout(r32f) readonly uniform image2D tex1;
            layout(rgba8) writeonly uniform image2D normal_tex;

            void main() {
                ivec2 i = ivec2(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);
                ivec2 ix1 = ivec2(min(i.x + 1, 255), i.y);
                ivec2 ix2 = ivec2(max(i.x - 1, 0), i.y);
                ivec2 iy1 = ivec2(i.x, min(i.y + 1, 255));
                ivec2 iy2 = ivec2(i.x, max(i.y - 1, 0));
                float y1 = imageLoad(tex1, ix1).x - imageLoad(tex1, ix2).x;
                float y2 = imageLoad(tex1, iy1).x - imageLoad(tex1, iy2).x;
                vec3 v1 = vec3(2.0 / 256.0, y1, 0.0);
                vec3 v2 = vec3(0.0, y2, 2.0 / 256.0);
                vec3 n = normalize(cross(v2.xyz, v1.xyz));
                imageStore(normal_tex, i, vec4(n, 0.0));
            }
            "#,
        )
        .unwrap();

        let bend_compute_shader = ComputeShader::from_source(
            display,
            r#"\
            #version 460 core
            layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

            layout(r32f) writeonly uniform image2D tex1;
            uniform int x;
            uniform int y;

            void main() {
                ivec2 i = ivec2(x, y);
                imageStore(tex1, i, vec4(-0.25, 0, 0, 0));
            }
            "#,
        )
        .unwrap();

        let n = 256f32;
        let h = 2.0 / (n - 1.0);
        let c = 1f32;
        let dt = 1.0 / n;
        let a = c * c * dt * dt / (h * h);
        let b = 2.0 - 4.0 * a;

        Self {
            height_compute_shader,
            swap_compute_shader,
            normal_compute_shader,
            bend_compute_shader,
            tex1,
            tex2,
            normal_tex,
            a,
            b,
            dt,
        }
    }

    pub fn compute(&self) {
        let tex1_unit = self
            .tex1
            .image_unit(ImageUnitFormat::R32F)
            .unwrap()
            .set_access(ImageUnitAccess::Read);
        let tex2_unit = self
            .tex2
            .image_unit(ImageUnitFormat::R32F)
            .unwrap()
            .set_access(ImageUnitAccess::ReadWrite);

        self.height_compute_shader.execute(
            uniform! {
                tex1: tex1_unit,
                tex2: tex2_unit,
                A: self.a,
                B: self.b,
            },
            32,
            64,
            1,
        );

        let tex1_unit = self
            .tex1
            .image_unit(ImageUnitFormat::R32F)
            .unwrap()
            .set_access(ImageUnitAccess::ReadWrite);
        let tex2_unit = self
            .tex2
            .image_unit(ImageUnitFormat::R32F)
            .unwrap()
            .set_access(ImageUnitAccess::ReadWrite);

        self.swap_compute_shader.execute(
            uniform! {
                tex1: tex1_unit,
                tex2: tex2_unit,
            },
            32,
            64,
            1,
        );

        let tex1_unit = self
            .tex1
            .image_unit(ImageUnitFormat::R32F)
            .unwrap()
            .set_access(ImageUnitAccess::Read);
        let normal_unit = self
            .normal_tex
            .image_unit(ImageUnitFormat::RGBA8)
            .unwrap()
            .set_access(ImageUnitAccess::Write);

        self.normal_compute_shader.execute(
            uniform! {
                tex1: tex1_unit,
                normal_tex: normal_unit,
            },
            32,
            64,
            1,
        );
    }

    pub fn bend(&self, x: i32, y: i32) {
        let tex1_unit = self
            .tex1
            .image_unit(ImageUnitFormat::R32F)
            .unwrap()
            .set_access(ImageUnitAccess::Write);

        self.bend_compute_shader.execute(
            uniform! {
                tex1: tex1_unit,
                x: x,
                y: y,
            },
            1,
            1,
            1,
        );
    }

    pub fn get_dt(&self) -> f32 {
        self.dt
    }
}
