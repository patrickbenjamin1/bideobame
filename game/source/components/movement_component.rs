use crate::core::geometry;
use wgpu::Buffer;

pub struct MovementComponent {
    pub velocity: geometry::Vector3,
    pub acceleration: geometry::Vector3,
    // @todo
    // pub _max_speed: f32,
}

impl MovementComponent {
    pub fn new(velocity: geometry::Vector3, acceleration: geometry::Vector3) -> Self {
        return Self {
            velocity: velocity,
            acceleration: acceleration,
        };
    }
}
