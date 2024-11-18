use crate::core::game::{ComponentEnum, ComponentType};
use crate::core::{game, renderer};

pub struct MovementSystem {}

impl game::System for MovementSystem {
    fn run(&self, world: &mut game::World, _renderer: &mut renderer::Renderer) {
        let entities_to_update = world
            .get_entities_with_components(&[ComponentType::Transform, ComponentType::Movement]);
        let delta_time = world.state().delta_time;

        for entity_id in entities_to_update {
            // Get movement data first
            let velocity;
            let acceleration;

            if let Some(ComponentEnum::Movement(movement)) =
                world.get_entity_component_by_type(entity_id, ComponentType::Movement)
            {
                velocity = movement.velocity;
                acceleration = movement.acceleration;
            } else {
                continue;
            }

            // Update transform with the collected movement data
            if let Some(ComponentEnum::Transform(transform)) = world
                .component_storage_mut()
                .get_component_mut(entity_id, |c| matches!(c, ComponentEnum::Transform(_)))
            {
                transform.translate([
                    velocity[0] * delta_time,
                    velocity[1] * delta_time,
                    velocity[2] * delta_time,
                ]);
            }

            // Finally update movement component with new velocity
            if let Some(ComponentEnum::Movement(movement)) = world
                .component_storage_mut()
                .get_component_mut(entity_id, |c| matches!(c, ComponentEnum::Movement(_)))
            {
                movement.velocity[0] += acceleration[0] * delta_time;
                movement.velocity[1] += acceleration[1] * delta_time;
                movement.velocity[2] += acceleration[2] * delta_time;
            }
        }
    }
}
