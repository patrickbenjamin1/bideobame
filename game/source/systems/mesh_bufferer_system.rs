use crate::core::{game, geometry, renderer};

/// System to buffer meshes for rendering
pub struct MeshBufferer {}

impl game::System for MeshBufferer {
    fn run(&self, world: &mut game::World, renderer: &mut renderer::Renderer) {
        let mesh_components = world.get_components_by_type_mut(game::ComponentType::Mesh);

        for (_, component) in mesh_components {
            if let game::ComponentTypes::Mesh(mesh_component) = component {
                if mesh_component.vertex_buffer.is_none() && mesh_component.needs_rebuffer {
                    let device = renderer.device();
                    let locked_device = device.lock().unwrap();

                    let vertices = mesh_component.last_vertices.as_ref().unwrap();
                    let indices = mesh_component.last_indices.as_ref().unwrap();

                    let vertex_buffer = locked_device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("Vertex Buffer"),
                        size: (std::mem::size_of::<geometry::Vertex>() * vertices.len())
                            as wgpu::BufferAddress,
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: true,
                    });

                    let index_buffer = locked_device.create_buffer(&wgpu::BufferDescriptor {
                        label: Some("Index Buffer"),
                        size: (std::mem::size_of::<u16>() * indices.len()) as wgpu::BufferAddress,
                        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                        mapped_at_creation: true,
                    });

                    // Write the data to the buffers
                    vertex_buffer
                        .slice(..)
                        .get_mapped_range_mut()
                        .copy_from_slice(bytemuck::cast_slice(vertices.as_slice()));
                    vertex_buffer.unmap();

                    index_buffer
                        .slice(..)
                        .get_mapped_range_mut()
                        .copy_from_slice(bytemuck::cast_slice(indices.as_slice()));
                    index_buffer.unmap();

                    mesh_component.vertex_buffer = Some(vertex_buffer);
                    mesh_component.index_buffer = Some(index_buffer);

                    mesh_component.needs_rebuffer = false;
                    mesh_component.num_indices = indices.len() as u32;
                }
            }
        }
    }
}
