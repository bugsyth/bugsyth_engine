use vek::Vec3;

pub mod aabb;
pub mod capsule;
pub mod sphere;

pub enum Shape {
    AABB(aabb::AABB),
    Sphere(sphere::Sphere),
    //Capsule(capsule::Capsule),
}

impl Shape {
    pub fn translate(&mut self, translation: Vec3<f32>) {
        match self {
            Self::AABB(aabb) => {
                aabb.min += translation;
                aabb.max += translation;
            }
            Self::Sphere(sphere) => sphere.center += translation,
        }
    }
}
