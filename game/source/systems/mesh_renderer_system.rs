use crate::components::mesh_component::MeshComponent;
use crate::core::{game, geometry, renderer};

use std::sync::{Arc, Mutex};

/// System to buffer meshes for rendering
pub struct MeshRenderer {}

impl game::RendererAccess for MeshRenderer {
    fn needs_renderer(&self) -> bool {
        true
    }
}

impl game::System for MeshRenderer {
    fn run(&self, world: &mut game::World, renderer: &mut renderer::Renderer) {
        let components = world.component_storage_mut().get_components_mut();

        for component in components.values_mut() {
            if let Some(mesh_component) = component.as_any_mut().downcast_mut::<MeshComponent>() {
                // queue the mesh for rendering
            }
        }
    }
}
