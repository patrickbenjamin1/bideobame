use crate::components::mesh_component;
use crate::core::renderer;
use crate::core::state;
use crate::systems::{mesh_bufferer_system, mesh_renderer_system};

use std::any::Any;
use std::collections::HashMap;
use std::sync::Mutex;

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

    pub fn add_component<T: Component + 'static>(&mut self, entity: EntityId, component: T) {
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
        let thing_entity = Entity::new();
        let entity_id = thing_entity.id;

        self.insert_entity(thing_entity);

        let (vertices, indices) = geometry::get_vertices(true);

        self.add_component(
            entity_id,
            mesh_component::MeshComponent::new(vertices, indices),
        );

        // Add systems to appropriate vectors
        self.add_update_system(mesh_bufferer_system::MeshBufferer {});
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
}
