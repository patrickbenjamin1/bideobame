use crate::components::collider_component;
use crate::components::mesh_component;
use crate::components::movement_component;
use crate::components::transform_component;

use crate::core::component_storage;
use crate::core::renderer;
use crate::core::state;

use crate::systems::movement_system;
use crate::systems::{collision_system, mesh_bufferer_system, mesh_renderer_system};

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

use super::geometry;

// Entity ID type
pub type EntityId = u32;

// Trait for components so that they can be stored in collections and downcasted
// Component-specific methods can be added here if needed

// Core entity struct, primarily for holding an ID
pub struct Entity {
    pub id: EntityId,
}

impl Entity {
    pub fn new() -> Self {
        let id = Self::generate_id();

        Self { id }
    }

    pub fn generate_id() -> EntityId {
        static NEXT_ID: AtomicU32 = AtomicU32::new(0);
        NEXT_ID.fetch_add(1, Ordering::SeqCst)
    }
}

// System trait for implementing systems that act on entities and components
pub trait System {
    fn run(&self, world: &mut World, renderer: &mut renderer::Renderer);
}

/// Storage for entities, components, and systems
pub struct World {
    entities: HashMap<EntityId, Entity>,
    component_storage: component_storage::ComponentStorage,
    update_systems: Vec<Box<dyn System>>,
    draw_systems: Vec<Box<dyn System>>,
    state: state::GameState,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            component_storage: component_storage::ComponentStorage::default(),
            update_systems: Vec::new(),
            draw_systems: Vec::new(),
            state: state::GameState::new(),
        }
    }

    pub fn insert_entity(&mut self, entity: Entity) -> &mut Self {
        self.entities.insert(entity.id, entity);

        return self;
    }

    pub fn add_update_system<T: System + 'static>(&mut self, system: T) -> &mut Self {
        self.update_systems.push(Box::new(system));

        return self;
    }

    pub fn add_draw_system<T: System + 'static>(&mut self, system: T) -> &mut Self {
        self.draw_systems.push(Box::new(system));

        return self;
    }

    pub fn run_update_systems(&mut self, renderer: &mut renderer::Renderer) -> &mut Self {
        let systems = std::mem::take(&mut self.update_systems);

        for system in systems.iter() {
            system.run(self, renderer);
        }

        self.update_systems = systems;

        return self;
    }

    pub fn run_draw_systems(&mut self, renderer: &mut renderer::Renderer) -> &mut Self {
        let systems = std::mem::take(&mut self.draw_systems);

        for system in systems.iter() {
            system.run(self, renderer);
        }

        self.draw_systems = systems;

        return self;
    }

    pub fn test_world(&mut self) {
        // create ground

        let ground_entity = Entity::new();
        let ground_entity_id = ground_entity.id;

        self.insert_entity(ground_entity);

        let (vertices, indices) = geometry::get_ground_quad();

        self.component_storage_mut().add_component(
            ground_entity_id,
            component_storage::ComponentTypes::Mesh(mesh_component::MeshComponent::new(
                vertices, indices,
            )),
        );

        self.component_storage_mut().add_component(
            ground_entity_id,
            component_storage::ComponentTypes::Transform(
                transform_component::TransformComponent::new(
                    [0.0, -2.0, 0.0],
                    [0.0, 0.0, 0.0],
                    [20.0, 20.0, 20.0],
                ),
            ),
        );

        // create cube

        let cube_entity = Entity::new();
        let cube_entity_id = cube_entity.id;

        self.insert_entity(cube_entity);

        let (vertices, indices) = geometry::get_cube();

        self.component_storage_mut().add_component(
            cube_entity_id,
            component_storage::ComponentTypes::Mesh(mesh_component::MeshComponent::new(
                vertices, indices,
            )),
        );

        self.component_storage_mut().add_component(
            cube_entity_id,
            component_storage::ComponentTypes::Transform(
                transform_component::TransformComponent::new(
                    [0.0, 0.5, -0.5],
                    [0.0, 0.0, 0.0],
                    [1.0, 1.0, 1.0],
                ),
            ),
        );

        self.component_storage_mut().add_component(
            cube_entity_id,
            component_storage::ComponentTypes::Movement(
                movement_component::MovementComponent::new([0.0, -1.0, 0.0], [0.0, 0.0, 0.0]),
            ),
        );

        self.add_update_system(mesh_bufferer_system::MeshBufferer {});
        self.add_update_system(movement_system::MovementSystem {});
        self.add_update_system(collision_system::CollisionSystem {});

        self.add_draw_system(mesh_renderer_system::MeshRenderer {});
    }

    // accessors

    pub fn entities(&self) -> &HashMap<EntityId, Entity> {
        &self.entities
    }

    pub fn component_storage(&self) -> &component_storage::ComponentStorage {
        &self.component_storage
    }

    pub fn component_storage_mut(&mut self) -> &mut component_storage::ComponentStorage {
        &mut self.component_storage
    }

    pub fn update_systems(&self) -> &Vec<Box<dyn System>> {
        &self.update_systems
    }

    pub fn draw_systems(&self) -> &Vec<Box<dyn System>> {
        &self.draw_systems
    }

    pub fn state(&mut self) -> &mut state::GameState {
        &mut self.state
    }
}
