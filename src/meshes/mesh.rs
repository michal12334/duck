use glium::glutin::surface::WindowSurface;
use glium::{Display, IndexBuffer, VertexBuffer};

use crate::meshes::vertex::Vertex;

#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, display: &Display<WindowSurface>) -> Self {
        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();
        Self {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
        }
    }
}
