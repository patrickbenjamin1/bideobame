use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Entity ID type
pub type EntityId = u32;

// Trait for components so that they can be stored in collections and downcasted
pub trait Component: Send + Sync {}
impl<T: Send + Sync> Component for T {}

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

// System trait for implementing systems that act on entities and components
pub trait System {
    fn run(&self, entities: &HashMap<EntityId, Entity>, components: &ComponentStorage);
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

    pub fn run_systems(&self) {
        for system in &self.systems {
            system.run(&self.entities, &self.component_storage);
        }
    }

    pub fn test_world(&mut self) {
        let thing_entity = Entity::new();

        self.insert_entity(thing_entity);
    }
}
