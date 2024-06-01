use glium::glutin::surface::WindowSurface;
use glium::{IndexBuffer, VertexBuffer};

use crate::water::vertex::Vertex;

#[derive(Debug)]
pub struct Water {
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u32>,
}

impl Water {
    pub fn new(display: &glium::Display<WindowSurface>) -> Self {
        let vertices = vec![
            Vertex::new([-1.0, 0.0, -1.0], [0.0, 0.0]),
            Vertex::new([1.0, 0.0, -1.0], [1.0, 0.0]),
            Vertex::new([-1.0, 0.0, 1.0], [0.0, 1.0]),
            Vertex::new([1.0, 0.0, 1.0], [1.0, 1.0]),
        ];

        let indices = vec![0, 1, 2, 2, 3, 1];

        let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
        let index_buffer = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &indices,
        )
        .unwrap();

        Self {
            vertex_buffer,
            index_buffer,
        }
    }
}
