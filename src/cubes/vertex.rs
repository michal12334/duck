use glium::implement_vertex;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub tex_index: u32,
}

implement_vertex!(Vertex, position, normal, tex_coords, tex_index);

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3], tex_coords: [f32; 2], tex_index: u32) -> Self {
        Vertex {
            position,
            normal,
            tex_coords,
            tex_index,
        }
    }
}
