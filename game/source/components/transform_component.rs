use crate::core::geometry;
use glam::{Mat4, Vec3, EulerRot};

pub struct TransformComponent {
    pub position: geometry::Vector3,
    pub rotation: geometry::Vector3,
    pub scale: geometry::Vector3,
    pub model_matrix: Mat4,
}

impl TransformComponent {
    pub fn new(
        position: geometry::Vector3,
        rotation: geometry::Vector3,
        scale: geometry::Vector3,
    ) -> Self {
        let model = Mat4::from_translation(Vec3::from_slice(&position)) 
            * Mat4::from_euler(EulerRot::XYZ, rotation[0], rotation[1], rotation[2])
            * Mat4::from_scale(Vec3::from_slice(&scale));

        Self {
            position,
            rotation,
            scale,
            model_matrix: model,
        }
    }

    // Helper to get matrix in wgpu-friendly format
    pub fn matrix_array(&self) -> [f32; 16] {
        self.model_matrix.to_cols_array()
    }

    pub fn translate(&mut self, translation: geometry::Vector3) {
        self.position[0] += translation[0];
        self.position[1] += translation[1];
        self.position[2] += translation[2];
    }

    pub fn rotate(&mut self, rotation: geometry::Vector3) {
        self.rotation[0] += rotation[0];
        self.rotation[1] += rotation[1];
        self.rotation[2] += rotation[2];
    }

    pub fn scale(&mut self, scale: geometry::Vector3) {
        self.scale[0] *= scale[0];
        self.scale[1] *= scale[1];
        self.scale[2] *= scale[2];
    }

    pub fn set_position(&mut self, position: geometry::Vector3) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: geometry::Vector3) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: geometry::Vector3) {
        self.scale = scale;
    }
}
