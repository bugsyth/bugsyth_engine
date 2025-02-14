use vek::Vec3;

use crate::physics::{
    physics_object::{ColliderType, PhysicsObject},
    shapes::{aabb::AABB, Shape},
};

pub fn solve_collision(obj: &mut PhysicsObject, other: &mut PhysicsObject) {
    let mtv = match (&obj.shape, &other.shape) {
        (Shape::AABB(obj), Shape::AABB(other)) => get_mtv_aabb_aabb(obj, other),
    };
    solve(obj, other, mtv);
}

fn solve(obj: &mut PhysicsObject, other: &mut PhysicsObject, mtv: Vec3<f32>) {
    match (&obj.collider_type, &other.collider_type) {
        (ColliderType::Dynamic, ColliderType::Dynamic) => {
            obj.shape.translate(mtv / 2.0);
            other.shape.translate(-mtv / 2.0);
        }
        (ColliderType::Dynamic, ColliderType::Static) => {
            obj.shape.translate(mtv);
        }
        (ColliderType::Static, ColliderType::Dynamic) => {
            other.shape.translate(-mtv);
        }
        (ColliderType::Static, ColliderType::Static) => {}
    }
}

fn get_mtv_aabb_aabb(obj: &AABB, other: &AABB) -> Vec3<f32> {
    let overlap = Vec3::new(
        (obj.max.x - other.min.x).min(other.max.x - obj.min.x),
        (obj.max.y - other.min.y).min(other.max.y - obj.min.y),
        (obj.max.z - other.min.z).min(other.max.z - obj.min.z),
    );

    if overlap.iter().all(|v| *v <= 0.0) {
        return Vec3::zero();
    }
    let min_overlap = overlap.x.min(overlap.y).min(overlap.z);

    let mut mtv = Vec3::zero();
    for i in 0..3 {
        if min_overlap == overlap[i] {
            mtv[i] = overlap[i] * if obj.min[i] < other.min[i] { -1.0 } else { 1.0 }
        }
    }
    mtv
}
