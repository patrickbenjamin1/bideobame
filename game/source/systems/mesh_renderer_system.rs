use crate::components::mesh_component::MeshComponent;
use crate::core::{game, renderer};

/// System to render meshes that have been buffered
pub struct MeshRenderer {}

impl game::System for MeshRenderer {
    fn run(&self, world: &mut game::World, renderer: &mut renderer::Renderer) {
        let output = match renderer.surface().get_current_texture() {
            Ok(output) => output,
            Err(e) => {
                println!("Failed to get surface texture: {:?}", e);
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = renderer.device().lock().unwrap().create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        // Get components before the render pass
        let components = world.component_storage_mut().get_components_mut();
        let mut mesh_count = 0;

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(renderer.render_pipeline());

            // Render each mesh component
            for component in components.values_mut() {
                if let Some(mesh_component) = component.as_any_mut().downcast_mut::<MeshComponent>()
                {
                    if let (Some(vertex_buffer), Some(index_buffer)) =
                        (&mesh_component.vertex_buffer, &mesh_component.index_buffer)
                    {
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass
                            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..mesh_component.num_indices, 0, 0..1);
                        mesh_count += 1;
                    }
                }
            }
        }

        renderer
            .queue()
            .lock()
            .unwrap()
            .submit(std::iter::once(encoder.finish()));

        output.present();

        // renderer.window().request_redraw();
    }
}
