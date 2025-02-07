use crate::context::Context;
use glium::winit::keyboard::KeyCode;
use std::f32::consts::PI;
use vek::{Mat4, Vec3};

pub struct CameraState {
    pub position: Vec3<f32>,
    pub target: Vec3<f32>,
    yaw: f32,
    pitch: f32,
    up: Vec3<f32>,
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
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

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let mat = Mat4::perspective_rh_no(self.fov, self.aspect_ratio, self.near, self.far);
        let cols = mat.cols;
        [
            [cols[0].x, cols[0].y, cols[0].z, cols[0].w],
            [cols[1].x, cols[1].y, cols[1].z, cols[1].w],
            [cols[2].x, cols[2].y, cols[2].z, cols[2].w],
            [cols[3].x, cols[3].y, cols[3].z, cols[3].w],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let mat = Mat4::look_at_rh(self.position, self.target, self.up);
        let cols = mat.cols;
        [
            [cols[0].x, cols[0].y, cols[0].z, cols[0].w],
            [cols[1].x, cols[1].y, cols[1].z, cols[1].w],
            [cols[2].x, cols[2].y, cols[2].z, cols[2].w],
            [cols[3].x, cols[3].y, cols[3].z, cols[3].w],
        ]
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

        if ctx.input.is_key_pressed(KeyCode::ArrowUp)
            && ctx.camera.pitch <= PI / 2.0 - 0.01 - cam_rot_speed / 1000.0
        {
            ctx.camera.pitch += cam_rot_speed / 1000.0 * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::ArrowLeft) {
            ctx.camera.yaw -= cam_rot_speed / 1000.0 * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::ArrowDown)
            && ctx.camera.pitch >= -PI / 2.0 + 0.01 + cam_rot_speed / 1000.0
        {
            ctx.camera.pitch -= cam_rot_speed / 1000.0 * dt;
        }
        if ctx.input.is_key_pressed(KeyCode::ArrowRight) {
            ctx.camera.yaw += cam_rot_speed / 1000.0 * dt;
        }

        /*

            Look around with mouse
            doesnt work well when using a gui lib

        // Handle mouse movement for camera rotation
        let window_center = PhysicalPosition::new(
            ctx.window.inner_size().width / 2,
            ctx.window.inner_size().height / 2,
        );
        let mouse_position = ctx.input.mouse_position();
        let delta_mouse = Vec2::new(
            window_center.x as f32 - mouse_position.x,
            window_center.y as f32 - mouse_position.y,
        );

        ctx.camera.yaw = -delta_mouse.x / 100.0 * cam_rot_speed;
        ctx.camera.pitch =
            (delta_mouse.y / 100.0 * cam_rot_speed).clamp(-PI / 2.0 + 0.01, PI / 2.0 - 0.01);
        */
    }
}
