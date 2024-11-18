use crate::core::{game, renderer};
use glam::{Mat4, Vec3};

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
            Err(_) => return,
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = renderer.device().lock().unwrap().create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        let mut current_transform_offset = 0;

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
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &renderer.depth_view(),
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(renderer.render_pipeline());
            render_pass.set_bind_group(0, renderer.global_bind_group(), &[]);

            // Get entities that have both Mesh and Transform components
            let entities = world.get_entities_with_components(&[
                game::ComponentType::Mesh,
                game::ComponentType::Transform,
            ]);

            for entity_id in entities {
                let mesh = world.get_entity_component_by_type(entity_id, game::ComponentType::Mesh);
                let transform =
                    world.get_entity_component_by_type(entity_id, game::ComponentType::Transform);

                if let (
                    Some(game::ComponentEnum::Mesh(mesh)),
                    Some(game::ComponentEnum::Transform(transform)),
                ) = (mesh, transform)
                {
                    if let (Some(vertex_buffer), Some(index_buffer)) =
                        (&mesh.vertex_buffer, &mesh.index_buffer)
                    {
                        // Update transform uniforms with the model matrix
                        renderer.update_transform_uniforms_at_offset(
                            renderer::TransformUniforms {
                                model: transform.matrix_array(),
                            },
                            current_transform_offset as wgpu::BufferAddress,
                        );

                        render_pass.set_bind_group(
                            1,
                            renderer.transform_bind_group(),
                            &[current_transform_offset],
                        );
                        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                        render_pass
                            .set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                        render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);

                        current_transform_offset +=
                            renderer::Renderer::get_transform_aligned_size() as u32;
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
