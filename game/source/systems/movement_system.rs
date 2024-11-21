use crate::core::game::{ComponentType, ComponentTypes};
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

            if let Some(ComponentTypes::Movement(movement)) =
                world.get_entity_component_by_type(entity_id, ComponentType::Movement)
            {
                velocity = movement.velocity;
                acceleration = movement.acceleration;
            } else {
                continue;
            }

            // Update transform with the collected movement data
            if let Some(ComponentTypes::Transform(transform)) = world
                .component_storage_mut()
                .get_component_mut(entity_id, |c| matches!(c, ComponentTypes::Transform(_)))
            {
                transform.translate([
                    velocity[0] * delta_time,
                    velocity[1] * delta_time,
                    velocity[2] * delta_time,
                ]);

                // check if there's a collision system on the entity
                if let Some(ComponentTypes::Collider(collider)) = world
                    .component_storage_mut()
                    .get_component_mut(entity_id, |c| matches!(c, ComponentTypes::Collider(_)))
                {
                    // tell the collider to update its bounds
                    collider.invalidate_bounds();
                }
            }

            // Finally update movement component with new velocity
            if let Some(ComponentTypes::Movement(movement)) = world
                .component_storage_mut()
                .get_component_mut(entity_id, |c| matches!(c, ComponentTypes::Movement(_)))
            {
                movement.velocity[0] += acceleration[0] * delta_time;
                movement.velocity[1] += acceleration[1] * delta_time;
                movement.velocity[2] += acceleration[2] * delta_time;
            }
        }
    }
}
