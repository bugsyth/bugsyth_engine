use vek::Vec3;

pub mod aabb;

pub enum Shape {
    AABB(aabb::AABB),
}

impl Shape {
    pub fn get_position(&self) -> Vec3<f32> {
        match self {
            Self::AABB(aabb) => aabb.min,
        }
    }
    pub fn translate(&mut self, translation: Vec3<f32>) {
        match self {
            Self::AABB(aabb) => {
                aabb.min += translation;
                aabb.max += translation;
            }
        }
    }
}
