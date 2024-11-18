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

pub fn get_ground_quad() -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u16> = vec![];

    // draw a flat quad

    vertices.push(Vertex {
        position: [-0.1, 0.0, -0.1],
        color: [1.0, 0.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, 0.0, -0.1],
        color: [0.0, 1.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, 0.0, 0.1],
        color: [0.0, 0.0, 1.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [-0.1, 0.0, 0.1],
        color: [1.0, 1.0, 0.0],
        should_wave: 0,
    });

    indices.push(0);
    indices.push(1);
    indices.push(2);
    indices.push(2);
    indices.push(3);
    indices.push(0);

    return (vertices, indices);
}

pub fn get_cube() -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices: Vec<Vertex> = vec![];
    let mut indices: Vec<u16> = vec![];

    // draw a cube

    // front
    vertices.push(Vertex {
        position: [-0.1, -0.1, -0.1],
        color: [1.0, 0.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, -0.1, -0.1],
        color: [0.0, 1.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, 0.1, -0.1],
        color: [0.0, 0.0, 1.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [-0.1, 0.1, -0.1],
        color: [1.0, 1.0, 0.0],
        should_wave: 0,
    });

    // back
    vertices.push(Vertex {
        position: [-0.1, -0.1, 0.1],
        color: [1.0, 0.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, -0.1, 0.1],
        color: [0.0, 1.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, 0.1, 0.1],
        color: [0.0, 0.0, 1.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [-0.1, 0.1, 0.1],
        color: [1.0, 1.0, 0.0],
        should_wave: 0,
    });

    // top
    vertices.push(Vertex {
        position: [-0.1, 0.1, -0.1],
        color: [1.0, 0.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, 0.1, -0.1],
        color: [0.0, 1.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, 0.1, 0.1],
        color: [0.0, 0.0, 1.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [-0.1, 0.1, 0.1],
        color: [1.0, 1.0, 0.0],
        should_wave: 0,
    });

    // bottom
    vertices.push(Vertex {
        position: [-0.1, -0.1, -0.1],
        color: [1.0, 0.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, -0.1, -0.1],
        color: [0.0, 1.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, -0.1, 0.1],
        color: [0.0, 0.0, 1.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [-0.1, -0.1, 0.1],
        color: [1.0, 1.0, 0.0],
        should_wave: 0,
    });

    // left
    vertices.push(Vertex {
        position: [-0.1, -0.1, -0.1],
        color: [1.0, 0.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [-0.1, -0.1, 0.1],
        color: [0.0, 1.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [-0.1, 0.1, 0.1],
        color: [0.0, 0.0, 1.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [-0.1, 0.1, -0.1],
        color: [1.0, 1.0, 0.0],
        should_wave: 0,
    });

    // right
    vertices.push(Vertex {
        position: [0.1, -0.1, -0.1],
        color: [1.0, 0.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, -0.1, 0.1],
        color: [0.0, 1.0, 0.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, 0.1, 0.1],
        color: [0.0, 0.0, 1.0],
        should_wave: 0,
    });
    vertices.push(Vertex {
        position: [0.1, 0.1, -0.1],
        color: [1.0, 1.0, 0.0],
        should_wave: 0,
    });

    // front
    indices.push(0);
    indices.push(1);
    indices.push(2);
    indices.push(2);
    indices.push(3);
    indices.push(0);

    // back
    indices.push(4);
    indices.push(5);
    indices.push(6);
    indices.push(6);
    indices.push(7);
    indices.push(4);

    // top
    indices.push(8);
    indices.push(9);
    indices.push(10);
    indices.push(10);
    indices.push(11);
    indices.push(8);

    // bottom
    indices.push(12);
    indices.push(13);
    indices.push(14);
    indices.push(14);
    indices.push(15);
    indices.push(12);

    // left
    indices.push(16);
    indices.push(17);
    indices.push(18);
    indices.push(18);
    indices.push(19);
    indices.push(16);

    // right
    indices.push(20);
    indices.push(21);
    indices.push(22);
    indices.push(22);
    indices.push(23);
    indices.push(20);

    (vertices, indices)
}
