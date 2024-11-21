use crate::components::{collider_component, mesh_component, transform_component};
use crate::core::game::{ComponentTypes, ComponentType};
use crate::core::geometry;
use crate::core::{game, renderer};

pub struct CollisionSystem {}

impl CollisionSystem {
    /// calculate the aabb for a mesh with its transform applied
    fn calculate_aabb(
        mesh_component: &mesh_component::MeshComponent,
        transform_component: &transform_component::TransformComponent,
    ) -> geometry::BoundingBox {
        let vertices = mesh_component.last_vertices.as_ref().unwrap();
        let mut min = [f32::MAX; 3];
        let mut max = [f32::MIN; 3];

        for vertex in vertices {
            let transformed_vertex = transform_component.apply_to_vertex(vertex);

            // loop through x, y, z
            for i in 0..3 {
                if transformed_vertex.position[i] < min[i] {
                    min[i] = transformed_vertex.position[i];
                }

                if transformed_vertex.position[i] > max[i] {
                    max[i] = transformed_vertex.position[i];
                }
            }
        }

        geometry::BoundingBox { min, max }
    }

    /// check if two bounding boxes intersect
    fn bounding_boxes_intersect(a: &geometry::BoundingBox, b: &geometry::BoundingBox) -> bool {
        for i in 0..3 {
            if a.max[i] < b.min[i] || a.min[i] > b.max[i] {
                return false;
            }
        }

        true
    }
}

impl game::System for CollisionSystem {
    fn run(&self, world: &mut game::World, _renderer: &mut renderer::Renderer) {
        // get all entities with colliders, transforms, and meshes
        let entities = world.get_entities_with_components(&[
            ComponentType::Collider,
            ComponentType::Transform,
            ComponentType::Mesh,
        ]);

        // loop through all entities with colliders, and update their aabbs if needed
        {
            for &entity_id in entities.iter() {
                // get the components we need
                let mut components = world.get_entity_components_mut(
                    entity_id,
                    &[
                        ComponentType::Collider,
                        ComponentType::Transform,
                        ComponentType::Mesh,
                    ],
                );

                if let [ComponentTypes::Collider(collider), ComponentTypes::Transform(transform), ComponentTypes::Mesh(mesh)] =
                    components.as_mut_slice()
                {
                    // if the collider needs to update its aabb, update it
                    if collider.needs_aabb_update {
                        collider.aabb = Some(CollisionSystem::calculate_aabb(&mesh, &transform));
                        collider.needs_aabb_update = false;
                    }
                }
            }
        }

        // loop through all entities with colliders, and check for collisions
        {
            for &entity_id in entities.iter() {
                // get the components we need from the entity
                let mut components = world.get_entity_components_mut(
                    entity_id,
                    &[
                        ComponentType::Collider,
                        ComponentType::Transform,
                        ComponentType::Mesh,
                    ],
                );

                if let [ComponentTypes::Collider(collider), ComponentTypes::Transform(transform), ComponentTypes::Mesh(mesh)] =
                    components.as_mut_slice()
                {
                    // loop through all entities again to check for collisions
                    for &other_entity_id in entities.iter() {
                        // don't check against self
                        if other_entity_id == entity_id {
                            continue;
                        }

                        // // get the components we need
                        // let mut other_components = world.get_entity_components_mut(
                        //     other_entity_id,
                        //     &[
                        //         ComponentType::Collider,
                        //         ComponentType::Transform,
                        //         ComponentType::Mesh,
                        //     ],
                        // );

                        // if let [ComponentTypes::Collider(other_collider), ComponentTypes::Transform(other_transform), ComponentTypes::Mesh(other_mesh)] =
                        //     other_components.as_mut_slice()
                        // {
                        //     // check for collision
                        //     if let Some(aabb) = &collider.aabb {
                        //         if let Some(other_aabb) = &other_collider.aabb {
                        //             if CollisionSystem::bounding_boxes_intersect(aabb, other_aabb) {
                        //                 // collision detected
                        //                 println!(
                        //                     "Collision detected between entities {} and {}",
                        //                     entity_id, other_entity_id
                        //                 );
                        //             }
                        //         }
                        //     }
                        // }
                    }
                }
            }
        }
    }
}
