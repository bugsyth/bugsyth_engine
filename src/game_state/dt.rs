use std::time::Instant;

pub struct DeltaTime {
    last_frame_time: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        Self {
            last_frame_time: Instant::now(),
        }
    }

    /// Run this at the start of every update
    pub fn get_dt(&mut self) -> f32 {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;
        dt
    }
}
