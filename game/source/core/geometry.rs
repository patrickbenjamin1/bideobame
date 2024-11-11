use bytemuck;
use std::sync::{Arc, Mutex};
use wgpu;

// type Vector2 = [f32; 2];
type Vector3 = [f32; 3];
type Colour = [f32; 3];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: Vector3,
    color: Colour,
    should_wave: u32,
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        return self.position == other.position
            && self.color == other.color
            && self.should_wave == other.should_wave;
    }
}

// describe the vertex layout for wgpu
impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress, // 1.
            step_mode: wgpu::VertexStepMode::Vertex,                            // 2.
            attributes: &[
                // 3.
                wgpu::VertexAttribute {
                    offset: 0,                             // 4.
                    shader_location: 0,                    // 5.
                    format: wgpu::VertexFormat::Float32x3, // 6.
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        };
    }
}

/// describes the positions of a mesh in the vertex buffer
pub struct MeshBufferPositions {
    vertex_offset: u64,
    index_offset: u64,
    vertex_length: u32,
    index_length: u32,
}

impl PartialEq for MeshBufferPositions {
    fn eq(&self, other: &Self) -> bool {
        // this should be a reliable way to compare meshes (there should never be overlapping offsets, if there is there's a bigger problem than this equality comparison)
        return self.vertex_offset == other.vertex_offset
            && self.index_offset == other.index_offset
            && self.vertex_length == other.vertex_length
            && self.index_length == other.index_length;
    }
}

impl Clone for MeshBufferPositions {
    fn clone(&self) -> Self {
        // this should be fine, because it's just values pointing to buffer positions
        return MeshBufferPositions {
            vertex_offset: self.vertex_offset,
            index_offset: self.index_offset,
            vertex_length: self.vertex_length,
            index_length: self.index_length,
        };
    }
}

impl MeshBufferPositions {
    /// take an array of vertices, insert them into the vertex and index buffer, keep details of them in the geometry manager
    /// and return a description of the mesh's position in the buffers
    pub fn create_with_vertices(
        geometry_manager: &mut GeometryManager,
        vertices: Vec<Vertex>,
        indices: Vec<u16>,
    ) -> MeshBufferPositions {
        let vertex_offset =
            geometry_manager.num_vertices() as u64 * std::mem::size_of::<Vertex>() as u64;
        let index_offset =
            geometry_manager.num_indices() as u64 * std::mem::size_of::<u16>() as u64;

        // lock queue for use
        let locked_queue = geometry_manager.queue.lock().unwrap();

        // add vertices to the buffer
        locked_queue.write_buffer(
            geometry_manager.vertex_buffer(),
            vertex_offset,
            bytemuck::cast_slice(vertices.as_slice()),
        );

        // add indices to the buffer
        locked_queue.write_buffer(
            geometry_manager.index_buffer(),
            index_offset,
            bytemuck::cast_slice(indices.as_slice()),
        );

        // update the number of vertices and indices
        geometry_manager.num_vertices += vertices.len() as u32;
        geometry_manager.num_indices += indices.len() as u32;

        // create the mesh object
        let mesh = MeshBufferPositions {
            vertex_offset,
            index_offset,
            vertex_length: vertices.len() as u32,
            index_length: indices.len() as u32,
        };

        // add it to geometry manager's meshes
        geometry_manager.meshes.push(mesh.clone());

        // return the mesh
        return mesh;
    }

    /// remove the mesh from the geometry manager
    pub fn remove(self, geometry_manager: &mut GeometryManager) {
        // remove vertices from the buffer
        let locked_queue = geometry_manager.queue.lock().unwrap();
        locked_queue.write_buffer(
            geometry_manager.vertex_buffer(),
            self.vertex_offset,
            bytemuck::cast_slice(&vec![
                0u8;
                self.vertex_length as usize
                    * std::mem::size_of::<Vertex>()
            ]),
        );

        // remove indices from the buffer
        locked_queue.write_buffer(
            geometry_manager.index_buffer(),
            self.index_offset,
            bytemuck::cast_slice(&vec![
                0u8;
                self.index_length as usize * std::mem::size_of::<u16>()
            ]),
        );

        // update the number of vertices and indices
        geometry_manager.num_vertices -= self.vertex_length;
        geometry_manager.num_indices -= self.index_length;

        // remove it from the geometry manager's meshes
        geometry_manager.meshes.retain(|m| m != &self);
    }
}

// manage the geometry
pub struct GeometryManager {
    // buffers
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_vertices: u32,
    num_indices: u32,

    // meshes
    meshes: Vec<MeshBufferPositions>,

    // wgpu stuff
    queue: Arc<Mutex<wgpu::Queue>>,
}

/**
 * @todo
 * - LOL NAH THIS IS UNNECESSARY - EACH MESH CAN HAVE ITS OWN BUFFER
 */

impl GeometryManager {
    const MAX_VERTEX_BUFFER_SIZE: usize = 1000000;
    const MAX_INDEX_BUFFER_SIZE: usize = 1000000;

    /// Create a new geometry manager
    pub fn init(device: Arc<Mutex<wgpu::Device>>, queue: Arc<Mutex<wgpu::Queue>>) -> Self {
        // get vertices (temporary for now lol)
        let (vertices, indices) = get_vertices(false);

        // lock arguments for use
        let locked_device = device.lock().unwrap();
        let locked_queue = queue.lock().unwrap();

        // create vertex buffer
        let vertex_buffer = locked_device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: GeometryManager::MAX_VERTEX_BUFFER_SIZE as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // add vertices to the buffer
        locked_queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(vertices.as_slice()));

        // create index buffer
        let index_buffer = locked_device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: GeometryManager::MAX_INDEX_BUFFER_SIZE as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // add indices to the buffer
        locked_queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(indices.as_slice()));

        return Self {
            vertex_buffer,
            index_buffer,
            num_vertices: vertices.len() as u32,
            num_indices: indices.len() as u32,
            queue: Arc::clone(&queue),
            meshes: vec![],
        };
    }

    /// Insert vertices into the geometry manager
    pub fn insert_mesh(&mut self, vertices: Vec<Vertex>, indices: Vec<u16>) -> MeshBufferPositions {
        let mesh = MeshBufferPositions::create_with_vertices(self, vertices, indices);

        return mesh;
    }

    /// Remove a meshes vertices from the geometry manager
    pub fn remove_mesh(&mut self, mesh: MeshBufferPositions) {
        mesh.remove(self);
    }

    pub fn remove_at_mesh_index(&mut self, index: usize) {
        if let Some(mesh) = self.meshes.get(index).cloned() {
            self.remove_mesh(mesh);
        }
    }

    // accessors for buffers

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        return &self.vertex_buffer;
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        return &self.index_buffer;
    }

    pub fn num_vertices(&self) -> u32 {
        return self.num_vertices;
    }

    pub fn num_indices(&self) -> u32 {
        return self.num_indices;
    }

    pub fn meshes(&self) -> &Vec<MeshBufferPositions> {
        return &self.meshes;
    }
}

/// STUFF FOR FUCKIN ROUND AND THAT

pub fn get_triangle(centre_x: f32, centre_y: f32, size: f32) -> Vec<Vertex> {
    let half_size = size / 2.0;

    return vec![
        Vertex {
            position: [centre_x, centre_y + half_size, 0.0],
            color: [1.0, 0.0, 0.0],
            should_wave: 1,
        },
        Vertex {
            position: [centre_x - half_size, centre_y - half_size, 0.0],
            color: [0.0, 1.0, 0.0],
            should_wave: 1,
        },
        Vertex {
            position: [centre_x + half_size, centre_y - half_size, 0.0],
            color: [0.0, 0.0, 1.0],
            should_wave: 1,
        },
    ];
}

pub fn get_vertices(add_five: bool) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u16> = vec![];

    for i in 0..100 {
        let y = (i / 10) as f32 * 0.1;
        let x = (i % 10) as f32 * 0.1;

        let triangle = get_triangle(x, y, 0.1);

        for (_, vertex) in triangle.iter().enumerate() {
            if let Some(index) = vertices.iter().position(|v| v == vertex) {
                indices.push(index as u16);
            } else {
                indices.push(vertices.len() as u16);
                vertices.push(*vertex);
            }
        }

        vertices.extend(triangle);
    }

    // for testing stuff
    if add_five {
        vertices = vertices
            .into_iter()
            .map(|v| Vertex {
                position: [v.position[0], v.position[1] + 0.1, v.position[2]],
                color: v.color,
                should_wave: v.should_wave,
            })
            .collect();
    }

    return (vertices, indices);
}
