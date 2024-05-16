use std::fs;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use crate::meshes::mesh::Mesh;
use crate::meshes::vertex::Vertex;

pub fn read_mesh(file_name: &str, display: &Display<WindowSurface>) -> Mesh {
    let content = fs::read_to_string(file_name).unwrap();
    let mut lines = content.lines();
    let vertices_count = lines.next().unwrap().parse::<usize>().unwrap();
    let mut vertices = Vec::with_capacity(vertices_count);
    for _ in 0..vertices_count {
        let line = lines.next().unwrap();
        let mut values = line.split_whitespace();
        let position = [
            values.next().unwrap().parse::<f32>().unwrap(),
            values.next().unwrap().parse::<f32>().unwrap(),
            values.next().unwrap().parse::<f32>().unwrap(),
        ];
        let normal = [
            values.next().unwrap().parse::<f32>().unwrap(),
            values.next().unwrap().parse::<f32>().unwrap(),
            values.next().unwrap().parse::<f32>().unwrap(),
        ];
        let tex_coords = [
            values.next().unwrap().parse::<f32>().unwrap(),
            values.next().unwrap().parse::<f32>().unwrap(),
        ];
        vertices.push(Vertex::new(position, normal, tex_coords));
    }
    
    let triangles_count = lines.next().unwrap().parse::<usize>().unwrap();
    let mut indices = Vec::with_capacity(triangles_count * 3);
    for _ in 0..triangles_count {
        let line = lines.next().unwrap();
        let mut values = line.split_whitespace();
        indices.push(values.next().unwrap().parse::<u32>().unwrap());
        indices.push(values.next().unwrap().parse::<u32>().unwrap());
        indices.push(values.next().unwrap().parse::<u32>().unwrap());
    }
    Mesh::new(vertices, indices, display)
}
