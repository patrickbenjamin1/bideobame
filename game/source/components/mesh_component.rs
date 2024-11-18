use crate::core::geometry;
use wgpu::Buffer;

pub struct MeshComponent {
    pub last_vertices: Option<Vec<geometry::Vertex>>,
    pub last_indices: Option<Vec<u16>>,
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub needs_rebuffer: bool,
    pub num_indices: u32,
}

impl MeshComponent {
    pub fn new(vertices: Vec<geometry::Vertex>, indices: Vec<u16>) -> Self {
        Self {
            last_vertices: Some(vertices),
            last_indices: Some(indices),
            vertex_buffer: None,
            index_buffer: None,
            needs_rebuffer: true,
            num_indices: 0,
        }
    }

    pub fn _update(&mut self, vertices: Vec<geometry::Vertex>, indices: Vec<u16>) {
        self.last_vertices = Some(vertices);
        self.last_indices = Some(indices);

        self.needs_rebuffer = true;
    }
}
