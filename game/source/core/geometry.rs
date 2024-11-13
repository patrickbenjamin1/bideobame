use bytemuck;
use wgpu;

// type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Colour = [f32; 3];

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
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                // Should wave
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }
}

/// STUFF FOR FUCKIN ROUND AND THAT

pub fn get_triangle(centre_x: f32, centre_y: f32, size: f32) -> Vec<Vertex> {
    let x = (centre_x * 2.0) - 1.0;
    let y = (centre_y * 2.0) - 1.0;
    let half_size = size;

    return vec![
        Vertex {
            position: [x, y + half_size, 0.0],
            color: [1.0, 0.0, 0.0], // Pure red
            should_wave: 1,
        },
        Vertex {
            position: [x - half_size, y - half_size, 0.0],
            color: [0.0, 1.0, 0.0], // Pure green
            should_wave: 1,
        },
        Vertex {
            position: [x + half_size, y - half_size, 0.0],
            color: [0.0, 0.0, 1.0], // Pure blue
            should_wave: 1,
        },
    ];
}

pub fn get_vertices(add_five: bool) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u16> = vec![];

    for i in 0..100 {
        let y = (i / 10) as f32 * 0.1; // Increased spacing
        let x = (i % 10) as f32 * 0.1; // Increased spacing

        let triangle = get_triangle(x, y, 0.1);

        // This part adds vertices and creates indices
        for vertex in triangle.iter() {
            if let Some(index) = vertices.iter().position(|v| v == vertex) {
                indices.push(index as u16);
            } else {
                indices.push(vertices.len() as u16);
                vertices.push(*vertex);
            }
        }
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
