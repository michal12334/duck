use glium::glutin::surface::WindowSurface;
use glium::{IndexBuffer, VertexBuffer};

use crate::cubes::vertex::Vertex;

#[derive(Debug)]
pub struct Cube {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub index_buffer: IndexBuffer<u32>,
}

impl Cube {
    pub fn new(display: &glium::Display<WindowSurface>) -> Self {
        let vertices = vec![
            // back
            Vertex::new([-1.0, -1.0, -1.0], [0.0, 0.0, 1.0], [0.0, 1.0], 0),
            Vertex::new([1.0, -1.0, -1.0], [0.0, 0.0, 1.0], [1.0, 1.0], 0),
            Vertex::new([1.0, 1.0, -1.0], [0.0, 0.0, 1.0], [1.0, 0.0], 0),
            Vertex::new([-1.0, 1.0, -1.0], [0.0, 0.0, 1.0], [0.0, 0.0], 0),
            // front
            Vertex::new([-1.0, -1.0, 1.0], [0.0, 0.0, -1.0], [0.0, 1.0], 0),
            Vertex::new([1.0, -1.0, 1.0], [0.0, 0.0, -1.0], [1.0, 1.0], 0),
            Vertex::new([1.0, 1.0, 1.0], [0.0, 0.0, -1.0], [1.0, 0.0], 0),
            Vertex::new([-1.0, 1.0, 1.0], [0.0, 0.0, -1.0], [0.0, 0.0], 0),
            // left
            Vertex::new([-1.0, -1.0, -1.0], [1.0, 0.0, 0.0], [0.0, 1.0], 0),
            Vertex::new([-1.0, -1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0], 0),
            Vertex::new([-1.0, 1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 0.0], 0),
            Vertex::new([-1.0, 1.0, -1.0], [1.0, 0.0, 0.0], [0.0, 0.0], 0),
            // right
            Vertex::new([1.0, -1.0, -1.0], [-1.0, 0.0, 0.0], [0.0, 1.0], 0),
            Vertex::new([1.0, -1.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 1.0], 0),
            Vertex::new([1.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 0.0], 0),
            Vertex::new([1.0, 1.0, -1.0], [-1.0, 0.0, 0.0], [0.0, 0.0], 0),
            // top
            Vertex::new([-1.0, 1.0, -1.0], [0.0, -1.0, 0.0], [0.0, 1.0], 1),
            Vertex::new([1.0, 1.0, -1.0], [0.0, -1.0, 0.0], [1.0, 1.0], 1),
            Vertex::new([1.0, 1.0, 1.0], [0.0, -1.0, 0.0], [1.0, 0.0], 1),
            Vertex::new([-1.0, 1.0, 1.0], [0.0, -1.0, 0.0], [0.0, 0.0], 1),
            // bottom
            Vertex::new([-1.0, -1.0, -1.0], [0.0, 1.0, 0.0], [0.0, 1.0], 2),
            Vertex::new([1.0, -1.0, -1.0], [0.0, 1.0, 0.0], [1.0, 1.0], 2),
            Vertex::new([1.0, -1.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0], 2),
            Vertex::new([-1.0, -1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0], 2),
        ];

        let indices = vec![
            // back
            0, 1, 2, 0, 2, 3, // front
            4, 6, 5, 4, 7, 6, // left
            8, 10, 9, 8, 11, 10, // right
            12, 13, 14, 12, 14, 15, // top
            16, 17, 18, 16, 18, 19, // bottom
            20, 22, 21, 20, 23, 22,
        ];

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
