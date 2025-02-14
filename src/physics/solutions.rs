use vek::Vec3;

use crate::physics::{
    physics_object::{ColliderType, PhysicsObject},
    shapes::{aabb::AABB, sphere::Sphere, Shape},
};

pub fn solve_collision(obj: &mut PhysicsObject, other: &mut PhysicsObject) {
    let mtv = match (&obj.shape, &other.shape) {
        (Shape::AABB(obj), Shape::AABB(other)) => get_mtv_aabb_aabb(obj, other),
        (Shape::Sphere(obj), Shape::Sphere(other)) => get_mtv_sphere_sphere(obj, other),
        (Shape::AABB(obj), Shape::Sphere(other)) => get_mtv_aabb_sphere(obj, other),
        (Shape::Sphere(other), Shape::AABB(obj)) => get_mtv_aabb_sphere(obj, other),
    };
    solve(obj, other, mtv);
}

fn solve(obj: &mut PhysicsObject, other: &mut PhysicsObject, mtv: Vec3<f32>) {
    match (&obj.collider_type, &other.collider_type) {
        (ColliderType::Dynamic, ColliderType::Dynamic) => {
            obj.shape.translate(mtv / 2.0);
            obj.shape.translate(-mtv / 2.0);
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

    if overlap.x <= 0.0 || overlap.y <= 0.0 || overlap.z <= 0.0 {
        println!("No collision, mtv = (0.0, 0.0, 0.0)");
        return Vec3::zero();
    }
    let min_overlap = overlap.x.min(overlap.y).min(overlap.z);

    if min_overlap == overlap.x {
        Vec3::new(
            overlap.x * if obj.min.x < other.min.x { -1.0 } else { 1.0 },
            0.0,
            0.0,
        )
    } else if min_overlap == overlap.y {
        Vec3::new(
            0.0,
            overlap.y * if obj.min.y < other.min.y { -1.0 } else { 1.0 },
            0.0,
        )
    } else if min_overlap == overlap.z {
        Vec3::new(
            0.0,
            0.0,
            overlap.z * if obj.min.z < other.min.z { -1.0 } else { 1.0 },
        )
    } else {
        println!("Failed to set min_overlap properly");
        Vec3::zero()
    }
}

fn get_mtv_sphere_sphere(obj: &Sphere, other: &Sphere) -> Vec3<f32> {
    todo!()
}

fn get_mtv_aabb_sphere(obj: &AABB, other: &Sphere) -> Vec3<f32> {
    todo!()
}
