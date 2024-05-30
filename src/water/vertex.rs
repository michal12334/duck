use glium::implement_vertex;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

impl Vertex {
    pub fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Vertex {
            position,
            normal,
        }
    }
}
