use crate::physics::Shape;

pub struct PhysicsObject {
    pub shape: Shape,
    pub collider_type: ColliderType,
}

impl PhysicsObject {
    pub fn new(shape: Shape, collider_type: ColliderType) -> Self {
        Self {
            shape,
            collider_type,
        }
    }
}

pub enum ColliderType {
    Static,
    Dynamic,
}
