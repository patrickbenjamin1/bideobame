use crate::components::{mesh_component::MeshComponent, transform_component::TransformComponent};
use crate::core::{game, renderer};
use glam::{Mat4, Vec3};

/// System to render meshes that have been buffered
pub struct MeshRenderer {}

impl game::System for MeshRenderer {
    fn run(&self, world: &mut game::World, renderer: &mut renderer::Renderer) {
        let state = world.state();

        // Create view and projection matrices
        let view = Mat4::look_at_rh(
            Vec3::new(0.0, 0.0, 5.0), // camera position
            Vec3::ZERO,               // look at point
            Vec3::Y,                  // up vector
        );

        let projection = Mat4::perspective_rh(
            45.0_f32.to_radians(),
            renderer.size().width as f32 / renderer.size().height as f32,
            0.1,
            100.0,
        );

        renderer.update_global_uniforms(renderer::GlobalUniforms {
            time: [state.total_time, state.delta_time, 0.0, 0.0],
            view: view.to_cols_array(),
            projection: projection.to_cols_array(),
        });

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

        // Get all components once before the loop
        let components = world.component_storage().get_components();

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
            render_pass.set_bind_group(0, renderer.global_bind_group(), &[]);

            // Render each mesh component
            for (entity_id, component) in components {
                // Try to get mesh component
                if let Some(mesh_component) = component.as_any().downcast_ref::<MeshComponent>() {
                    // Look for a transform component on the same entity
                    let transform = components
                        .get(entity_id)
                        .and_then(|c| c.as_any().downcast_ref::<TransformComponent>());

                    if let (Some(vertex_buffer), Some(index_buffer)) =
                        (&mesh_component.vertex_buffer, &mesh_component.index_buffer)
                    {
                        // Update per-object uniforms if needed
                        // renderer.update_model_matrix(model_matrix.to_cols_array());

                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass
                            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..mesh_component.num_indices, 0, 0..1);
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

        renderer.window().request_redraw();
    }
}
