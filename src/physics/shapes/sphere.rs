use vek::Vec3;

pub struct Sphere {
    pub center: Vec3<f32>,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }
}
