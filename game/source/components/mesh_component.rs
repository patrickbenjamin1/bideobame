use crate::core::{game, geometry};

use wgpu;

pub struct MeshComponent {
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
    pub num_indices: u32,

    pub needs_rebuffer: bool,
    pub last_vertices: Option<Vec<geometry::Vertex>>,
    pub last_indices: Option<Vec<u16>>,
}

impl MeshComponent {
    pub fn new(vertices: Vec<geometry::Vertex>, indices: Vec<u16>) -> Self {
        let num_indices = indices.len() as u32;
        let needs_rebuffer = true;

        Self {
            vertex_buffer: None,
            index_buffer: None,
            num_indices,
            needs_rebuffer,
            last_vertices: Some(vertices),
            last_indices: Some(indices),
        }
    }

    pub fn _update(&mut self, vertices: Vec<geometry::Vertex>, indices: Vec<u16>) {
        self.last_vertices = Some(vertices);
        self.last_indices = Some(indices);

        self.needs_rebuffer = true;
    }
}
