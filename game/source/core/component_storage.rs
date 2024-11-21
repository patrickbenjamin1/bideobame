use crate::components::collider_component;
use crate::components::mesh_component;
use crate::components::movement_component;
use crate::components::transform_component;

use crate::core::game;

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    Mesh,
    Transform,
    Movement,
    Collider,
}

// Define an enum to hold different component types
pub enum ComponentTypes {
    Mesh(mesh_component::MeshComponent),
    Transform(transform_component::TransformComponent),
    Movement(movement_component::MovementComponent),
    Collider(collider_component::ColliderComponent),
}

impl ComponentTypes {
    pub fn component_type(&self) -> ComponentType {
        match self {
            ComponentTypes::Mesh(_) => ComponentType::Mesh,
            ComponentTypes::Transform(_) => ComponentType::Transform,
            ComponentTypes::Movement(_) => ComponentType::Movement,
            ComponentTypes::Collider(_) => ComponentType::Collider,
        }
    }
}

// Update the ComponentStorage to use ComponentTypes
#[derive(Default)]
pub struct ComponentStorage {
    components: HashMap<game::EntityId, Vec<RefCell<ComponentTypes>>>,
}

impl ComponentStorage {
    // Method to add a component to a specific entity
    pub fn add_component(
        &mut self,
        entity: game::EntityId,
        component: ComponentTypes,
    ) -> &mut Self {
        self.components
            .entry(entity)
            .or_default()
            .push(RefCell::new(component));
        return self;
    }

    /// run a closure on each component of a specific entity
    pub fn foreach_component_by_type<F>(
        &mut self,
        component_type: ComponentType,
        mut f: F,
    ) -> &mut Self
    where
        F: FnMut(&ComponentTypes),
    {
        for components in self.components.values() {
            for component in components {
                if component.borrow().component_type() == component_type {
                    f(&component.borrow());
                }
            }
        }

        return self;
    }

    /// run a closure on each component on a given entity
    pub fn foreach_component_by_entity<F>(&mut self, entity: game::EntityId, mut f: F) -> &mut Self
    where
        F: FnMut(&ComponentTypes),
    {
        if let Some(components) = self.components.get(&entity) {
            for component in components {
                f(&component.borrow());
            }
        }

        return self;
    }

    /// run a closure on a component of a specific type on a specific entity
    pub fn with_component(
        &mut self,
        entity: game::EntityId,
        component_type: ComponentType,
        f: impl Fn(&ComponentTypes),
    ) -> &mut Self {
        if let Some(components) = self.components.get(&entity) {
            for component in components {
                if component.borrow().component_type() == component_type {
                    f(&component.borrow());
                }
            }
        }

        return self;
    }

    /// run a closure on each entity which has a specific component type
    pub fn foreach_entity_with_component_types<F>(
        &mut self,
        component_types: Vec<ComponentType>,
        mut f: F,
    ) -> &mut Self
    where
        F: FnMut(game::EntityId, &Vec<RefCell<ComponentTypes>>),
    {
        for (entity, components) in self.components.iter() {
            let mut has_all_components = true;

            for component_type in component_types.iter() {
                let mut has_component = false;

                for component in components.iter() {
                    if component.borrow().component_type() == *component_type {
                        has_component = true;
                        break;
                    }
                }

                if !has_component {
                    has_all_components = false;
                    break;
                }
            }

            if has_all_components {
                f(*entity, components);
            }
        }

        return self;
    }

    // @note - this assumes that each entity will only have one component of each type - revisit if this becomes a problem
    pub fn remove_component(
        &mut self,
        entity: game::EntityId,
        component_type: ComponentType,
    ) -> &mut Self {
        if let Some(components) = self.components.get_mut(&entity) {
            components.retain(|component| component.borrow().component_type() != component_type);
        }

        return self;
    }

    pub fn remove_entity(&mut self, entity: game::EntityId) -> &mut Self {
        self.components.remove(&entity);
        return self;
    }
}

fn main() {
    let mut component_storage = RefCell::new(ComponentStorage::default());

    component_storage
        .borrow_mut()
        .foreach_entity_with_component_types(vec![ComponentType::Mesh], |entity, components| {
            component_storage.borrow_mut().with_component(
                entity,
                ComponentType::Movement,
                |movement_component| {
                    // do something with the movement component

                    component_storage.borrow_mut().with_component(
                        entity,
                        ComponentType::Transform,
                        |transform_component| {
                            return ();
                        },
                    );
                },
            );
        });
}
