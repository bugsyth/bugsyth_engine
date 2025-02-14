use vek::Vec3;

pub struct Capsule {
    top: Vec3<f32>,
    bottom: Vec3<f32>,
    radius: f32,
}

impl Capsule {
    pub fn new(top: Vec3<f32>, bottom: Vec3<f32>, radius: f32) -> Self {
        Self {
            top,
            bottom,
            radius,
        }
    }
}
