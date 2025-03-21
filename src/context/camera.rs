use crate::context::Context;
use glium::winit::keyboard::KeyCode;
use std::f32::consts::PI;
use vek::{Mat4, Vec3};

/// Basic camera state held inside the `Context`
pub struct CameraState {
    pub position: Vec3<f32>,
    target: Vec3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub up: Vec3<f32>,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraState {
    pub fn new(
        position: Vec3<f32>,
        target: Vec3<f32>,
        up: Vec3<f32>,
        fov: f32,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> CameraState {
        CameraState {
            position,
            target,
            yaw: 0.0,
            pitch: 0.0,
            up,
            fov,
            aspect_ratio,
            near,
            far,
        }
    }

    pub fn look_at(&mut self, target: Vec3<f32>) {
        let direction = (target - self.position).normalized();

        self.pitch = direction.y.asin();
        self.yaw = direction.x.atan2(direction.z);
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        Mat4::perspective_rh_no(self.fov, self.aspect_ratio, self.near, self.far).into_col_arrays()
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        Mat4::look_at_rh(self.position, self.target, self.up).into_col_arrays()
    }

    /// Run at the end of any game update loop
    pub fn update(&mut self) {
        self.target = Vec3::new(
            self.position.x + self.yaw.cos() * self.pitch.cos(),
            self.position.y + self.pitch.sin(),
            self.position.z + self.yaw.sin() * self.pitch.cos(),
        );
    }

    fn get_directions(&self) -> (Vec3<f32>, Vec3<f32>, Vec3<f32>) {
        let f = (self.target - self.position).normalized();
        let r = self.up.cross(f).normalized();
        let u = f.cross(r).normalized();
        (f, r, u)
    }

    /// WASD movement, left shift down, space up, mouse for turning and free the mouse by holding E.
    /// Not built for preformance, here for debugging
    pub fn free_cam(dt: f32, ctx: &mut Context, cam_speed: f32, cam_rot_speed: f32) {
        let (f, s, _) = ctx.camera.get_directions();
        // Handle camera movement
        if ctx.input.is_key_pressed(KeyCode::KeyW) {
            ctx.camera.position += f * cam_speed * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::KeyA) {
            ctx.camera.position += s * cam_speed * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::KeyS) {
            ctx.camera.position -= f * cam_speed * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::KeyD) {
            ctx.camera.position -= s * cam_speed * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::Space) {
            ctx.camera.position.y += cam_speed * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::ShiftLeft) {
            ctx.camera.position.y -= cam_speed * dt;
        }

        let cam_rot = cam_rot_speed * dt;
        if ctx.input.is_key_pressed(KeyCode::ArrowUp)
            && ctx.camera.pitch <= PI / 2.0 - 0.01 - cam_rot
        {
            ctx.camera.pitch += cam_rot;
        }
        if ctx.input.is_key_pressed(KeyCode::ArrowLeft) {
            ctx.camera.yaw -= cam_rot;
        }
        if ctx.input.is_key_pressed(KeyCode::ArrowDown)
            && ctx.camera.pitch >= -PI / 2.0 + 0.01 + cam_rot
        {
            ctx.camera.pitch -= cam_rot;
        }
        if ctx.input.is_key_pressed(KeyCode::ArrowRight) {
            ctx.camera.yaw += cam_rot;
        }

        // Handle mouse movement for camera rotation
        let mouse_look = !ctx.input.is_key_pressed(KeyCode::KeyE);
        ctx.input.lock_mouse_near_center(&ctx.window, mouse_look);

        if !mouse_look {
            return;
        }
        let delta_mouse = ctx.input.delta_mouse();
        ctx.camera.yaw += -delta_mouse.x / 100.0 * cam_rot_speed;
        ctx.camera.pitch += delta_mouse.y / 100.0 * cam_rot_speed;
        ctx.camera.pitch = ctx.camera.pitch.clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);
    }
}
