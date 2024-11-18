use crate::core::geometry;

pub struct ColliderComponent {
    pub aabb: Option<geometry::BoundingBox>,
    pub obb: Option<geometry::BoundingBox>,
    pub needs_aabb_update: bool,
    pub needs_obb_update: bool,
}

impl ColliderComponent {
    pub fn new() -> Self {
        Self {
            aabb: None,
            obb: None,
            needs_aabb_update: true,
            needs_obb_update: true,
        }
    }

    pub fn invalidate_bounds(&mut self) {
        self.needs_aabb_update = true;
        self.needs_obb_update = true;
    }
}
