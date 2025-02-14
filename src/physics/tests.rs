use crate::physics::shapes::{aabb::AABB, sphere::Sphere, Shape};
mod support;

pub fn test_collision(obj: &Shape, other: &Shape) -> bool {
    match (obj, other) {
        (Shape::AABB(obj), Shape::AABB(other)) => test_aabb_aabb(obj, other),
        (Shape::Sphere(obj), Shape::Sphere(other)) => test_sphere_sphere(obj, other),
        (Shape::AABB(obj), Shape::Sphere(other)) => test_aabb_sphere(obj, other),
        (Shape::Sphere(other), Shape::AABB(obj)) => test_aabb_sphere(obj, other),
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

fn test_sphere_sphere(obj: &Sphere, other: &Sphere) -> bool {
    obj.center.distance_squared(other.center) <= (obj.radius + other.radius).powi(2)
}

fn test_aabb_sphere(obj: &AABB, other: &Sphere) -> bool {
    other
        .center
        .map3(obj.min, obj.max, |c, min, max| c.clamp(min, max))
        .distance_squared(other.center)
        <= other.radius.powi(2)
}
