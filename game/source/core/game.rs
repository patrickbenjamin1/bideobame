use crate::components::collider_component;
use crate::components::mesh_component;
use crate::components::movement_component;
use crate::components::transform_component;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    Mesh,
    Transform,
    Movement,
    Collider,
}

// Define an enum to hold different component types
pub enum ComponentEnum {
    Mesh(mesh_component::MeshComponent),
    Transform(transform_component::TransformComponent),
    Movement(movement_component::MovementComponent),
    Collider(collider_component::ColliderComponent),
    // Add other component types here
}

impl ComponentEnum {
    pub fn component_type(&self) -> ComponentType {
        match self {
            ComponentEnum::Mesh(_) => ComponentType::Mesh,
            ComponentEnum::Transform(_) => ComponentType::Transform,
            ComponentEnum::Movement(_) => ComponentType::Movement,
            ComponentEnum::Collider(_) => ComponentType::Collider,
        }
    }
}

// Update the ComponentStorage to use ComponentEnum
#[derive(Default)]
pub struct ComponentStorage {
    components: HashMap<EntityId, Vec<ComponentEnum>>,
}

impl ComponentStorage {
    // Method to add a component to a specific entity
    pub fn add_component(&mut self, entity: EntityId, component: ComponentEnum) {
        self.components.entry(entity).or_default().push(component);
    }

    pub fn get_components(&self) -> &HashMap<EntityId, Vec<ComponentEnum>> {
        &self.components
    }

    pub fn get_components_mut(&mut self) -> &mut HashMap<EntityId, Vec<ComponentEnum>> {
        &mut self.components
    }

    pub fn get_component(
        &self,
        entity: EntityId,
        component_type: fn(&ComponentEnum) -> bool,
    ) -> Option<&ComponentEnum> {
        self.components
            .get(&entity)?
            .iter()
            .find(|c| component_type(c))
    }

    pub fn get_component_mut(
        &mut self,
        entity: EntityId,
        component_type: fn(&ComponentEnum) -> bool,
    ) -> Option<&mut ComponentEnum> {
        self.components
            .get_mut(&entity)?
            .iter_mut()
            .find(|c| component_type(c))
    }

    pub fn get_components_by_type(
        &self,
        component_type: ComponentType,
    ) -> Vec<(EntityId, &ComponentEnum)> {
        self.components
            .iter()
            .flat_map(|(entity_id, components)| {
                components
                    .iter()
                    .filter(move |c| c.component_type() == component_type)
                    .map(move |c| (*entity_id, c))
            })
            .collect()
    }

    pub fn get_components_by_type_mut(
        &mut self,
        component_type: ComponentType,
    ) -> Vec<(EntityId, &mut ComponentEnum)> {
        self.components
            .iter_mut()
            .flat_map(|(entity_id, components)| {
                components
                    .iter_mut()
                    .filter(move |c| c.component_type() == component_type)
                    .map(move |c| (*entity_id, c))
            })
            .collect()
    }

    pub fn get_entity_components(&self, entity_id: EntityId) -> Option<&Vec<ComponentEnum>> {
        self.components.get(&entity_id)
    }

    pub fn get_entity_component_by_type(
        &self,
        entity_id: EntityId,
        component_type: ComponentType,
    ) -> Option<&ComponentEnum> {
        self.components
            .get(&entity_id)?
            .iter()
            .find(|c| c.component_type() == component_type)
    }

    pub fn get_entities_with_components(&self, required_types: &[ComponentType]) -> Vec<EntityId> {
        self.components
            .iter()
            .filter(|(_, components)| {
                required_types.iter().all(|required_type| {
                    components
                        .iter()
                        .any(|c| c.component_type() == *required_type)
                })
            })
            .map(|(entity_id, _)| *entity_id)
            .collect()
    }

    pub fn get_components_by_types_mut<'a>(
        &'a mut self,
        entity_id: EntityId,
        types: &[ComponentType],
    ) -> Vec<&'a mut ComponentEnum> {
        if let Some(components) = self.components.get_mut(&entity_id) {
            // Safe because we know we're getting different component types
            let ptrs: Vec<*mut ComponentEnum> = components
                .iter_mut()
                .filter(|c| types.contains(&c.component_type()))
                .map(|c| c as *mut ComponentEnum)
                .collect();

            // Convert raw pointers back to references
            return ptrs.into_iter().map(|p| unsafe { &mut *p }).collect();
        }
        Vec::new()
    }
}

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
    component_storage: ComponentStorage,
    update_systems: Vec<Box<dyn System>>,
    draw_systems: Vec<Box<dyn System>>,
    state: state::GameState,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            component_storage: ComponentStorage::default(),
            update_systems: Vec::new(),
            draw_systems: Vec::new(),
            state: state::GameState::new(),
        }
    }

    pub fn insert_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    pub fn add_component(&mut self, entity: EntityId, component: ComponentEnum) {
        self.component_storage.add_component(entity, component);
    }

    pub fn add_update_system<T: System + 'static>(&mut self, system: T) {
        self.update_systems.push(Box::new(system));
    }

    pub fn add_draw_system<T: System + 'static>(&mut self, system: T) {
        self.draw_systems.push(Box::new(system));
    }

    pub fn run_update_systems(&mut self, renderer: &mut renderer::Renderer) {
        let systems = std::mem::take(&mut self.update_systems);

        for system in systems.iter() {
            system.run(self, renderer);
        }

        self.update_systems = systems;
    }

    pub fn run_draw_systems(&mut self, renderer: &mut renderer::Renderer) {
        let systems = std::mem::take(&mut self.draw_systems);
        for system in systems.iter() {
            system.run(self, renderer);
        }
        self.draw_systems = systems;
    }

    pub fn test_world(&mut self) {
        // create ground

        let ground_entity = Entity::new();
        let ground_entity_id = ground_entity.id;

        self.insert_entity(ground_entity);

        let (vertices, indices) = geometry::get_ground_quad();

        self.add_component(
            ground_entity_id,
            ComponentEnum::Mesh(mesh_component::MeshComponent::new(vertices, indices)),
        );

        self.add_component(
            ground_entity_id,
            ComponentEnum::Transform(transform_component::TransformComponent::new(
                [0.0, -2.0, 0.0],
                [0.0, 0.0, 0.0],
                [20.0, 20.0, 20.0],
            )),
        );

        // create cube

        let cube_entity = Entity::new();
        let cube_entity_id = cube_entity.id;

        self.insert_entity(cube_entity);

        let (vertices, indices) = geometry::get_cube();

        self.add_component(
            cube_entity_id,
            ComponentEnum::Mesh(mesh_component::MeshComponent::new(vertices, indices)),
        );

        self.add_component(
            cube_entity_id,
            ComponentEnum::Transform(transform_component::TransformComponent::new(
                [0.0, 0.5, -0.5],
                [0.0, 0.0, 0.0],
                [1.0, 1.0, 1.0],
            )),
        );

        self.add_component(
            cube_entity_id,
            ComponentEnum::Movement(movement_component::MovementComponent::new(
                [0.0, -1.0, 0.0],
                [0.0, 0.0, 0.0],
            )),
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

    pub fn component_storage(&self) -> &ComponentStorage {
        &self.component_storage
    }

    pub fn component_storage_mut(&mut self) -> &mut ComponentStorage {
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

    // Convenience methods that wrap ComponentStorage queries
    pub fn get_components_by_type(
        &self,
        component_type: ComponentType,
    ) -> Vec<(EntityId, &ComponentEnum)> {
        self.component_storage
            .get_components_by_type(component_type)
    }

    pub fn get_components_by_type_mut(
        &mut self,
        component_type: ComponentType,
    ) -> Vec<(EntityId, &mut ComponentEnum)> {
        self.component_storage_mut()
            .get_components_by_type_mut(component_type)
    }

    pub fn get_entity_components(&self, entity_id: EntityId) -> Option<&Vec<ComponentEnum>> {
        self.component_storage.get_entity_components(entity_id)
    }

    pub fn get_entity_component_by_type(
        &self,
        entity_id: EntityId,
        component_type: ComponentType,
    ) -> Option<&ComponentEnum> {
        self.component_storage
            .get_entity_component_by_type(entity_id, component_type)
    }

    pub fn get_entities_with_components(&self, required_types: &[ComponentType]) -> Vec<EntityId> {
        self.component_storage
            .get_entities_with_components(required_types)
    }

    pub fn get_entity_components_mut(
        &mut self,
        entity_id: EntityId,
        types: &[ComponentType],
    ) -> Vec<&mut ComponentEnum> {
        self.component_storage_mut()
            .get_components_by_types_mut(entity_id, types)
    }
}
