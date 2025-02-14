use vek::Vec3;

pub struct AABB {
    pub min: Vec3<f32>,
    pub max: Vec3<f32>,
}

impl AABB {
    pub fn new(min: Vec3<f32>, max: Vec3<f32>) -> Self {
        Self { min, max }
    }
}
