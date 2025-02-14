use crate::physics::shapes::{aabb::AABB, Shape};

pub fn test_collision(obj: &Shape, other: &Shape) -> bool {
    match (obj, other) {
        (Shape::AABB(obj), Shape::AABB(other)) => test_aabb_aabb(obj, other),
    }
}

fn test_aabb_aabb(obj: &AABB, other: &AABB) -> bool {
    !(obj.max.x < other.min.x
        || obj.min.x > other.max.x
        || obj.max.y < other.min.y
        || obj.min.y > other.max.y
        || obj.max.z < other.min.z
        || obj.min.z > other.max.z)
}
