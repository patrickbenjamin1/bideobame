use crate::components::transform_component::TransformComponent;
use crate::core::{game, renderer};
use glam::Mat4;

pub struct TransformSystem {}

impl game::System for TransformSystem {
    fn run(&self, world: &mut game::World, _renderer: &mut renderer::Renderer) {
        let components = world.component_storage_mut().get_components_mut();

        // Update transform matrices for all transform components
        for component in components.values_mut() {
            if let Some(transform) = component.as_any_mut().downcast_mut::<TransformComponent>() {
                let translation = Mat4::from_translation(transform.position.into());
                let rotation = Mat4::from_euler(
                    glam::EulerRot::XYZ,
                    transform.rotation[0],
                    transform.rotation[1],
                    transform.rotation[2],
                );
                let scale = Mat4::from_scale(transform.scale.into());

                transform.model_matrix = translation * rotation * scale;
            }
        }
    }
}
