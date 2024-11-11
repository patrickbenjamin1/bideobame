use crate::components::mesh_component;
use crate::core::renderer;

use crate::systems::mesh_bufferer_system;

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wgpu;

use super::geometry;

// Entity ID type
pub type EntityId = u32;

// Trait for components so that they can be stored in collections and downcasted
pub trait Component: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any + Send + Sync> Component for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Component storage structure
#[derive(Default)]
pub struct ComponentStorage {
    components: HashMap<EntityId, Box<dyn Component>>,
}

impl ComponentStorage {
    // Method to add a component to a specific entity
    pub fn add_component<T: Component + 'static>(&mut self, entity: EntityId, component: T) {
        self.components.insert(entity, Box::new(component));
    }

    // pub fn get_by_entity<T: Component + 'static>(&self, entity: EntityId) -> Option<&T> {
    //     self.components.get(&entity).and_then(|c| c.downcast_ref())
    // }

    pub fn get_components(&self) -> &HashMap<EntityId, Box<dyn Component>> {
        &self.components
    }

    pub fn get_components_mut(&mut self) -> &mut HashMap<EntityId, Box<dyn Component>> {
        &mut self.components
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
        static NEXT_ID: Mutex<EntityId> = Mutex::new(0);

        let mut id = NEXT_ID.lock().unwrap();
        *id += 1;

        *id
    }
}

// New trait to define if a system needs access to the renderer
pub trait RendererAccess {
    /// Should just return true or false depending on if the system implementing this trait needs access to the renderer
    fn needs_renderer(&self) -> bool;
}

// System trait for implementing systems that act on entities and components
pub trait System: RendererAccess {
    fn run(&self, world: &mut World, renderer: &mut renderer::Renderer);
}

/// Storage for entities, components, and systems
pub struct World {
    entities: HashMap<EntityId, Entity>,
    component_storage: ComponentStorage,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            component_storage: ComponentStorage::default(),
            systems: Vec::new(),
        }
    }

    pub fn insert_entity(&mut self, entity: Entity) {
        self.entities.insert(entity.id, entity);
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: EntityId, component: T) {
        self.component_storage.add_component(entity, component);
    }

    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn run_systems(&mut self, renderer: &mut renderer::Renderer) {
        // Take ownership of systems temporarily
        let systems = std::mem::take(&mut self.systems);

        // Run each system
        for system in systems.iter() {
            system.run(self, renderer);
        }

        // Put systems back
        self.systems = systems;
    }

    pub fn test_world(&mut self) {
        let thing_entity = Entity::new();
        let entity_id = thing_entity.id;

        self.insert_entity(thing_entity);

        let (vertices, indices) = geometry::get_vertices(true);

        self.add_component(
            entity_id,
            mesh_component::MeshComponent::new(vertices, indices),
        );

        self.add_system(mesh_bufferer_system::MeshBufferer {});
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

    pub fn systems(&self) -> &Vec<Box<dyn System>> {
        &self.systems
    }
}
