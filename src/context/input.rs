use crate::error::EngineResult;
use glium::winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, Window},
};
use std::collections::HashMap;
use vek::Vec2;

#[derive(Debug, Default)]
pub struct Input {
    keys: HashMap<KeyCode, ElementState>,
    mouse_buttons: HashMap<MouseButton, ElementState>,
    last_mouse_position: Vec2<f32>,
    delta_mouse: Vec2<f32>,
    mouse_position_set: bool,
}

impl Input {
    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        if let Some(state) = self.keys.get(&key_code) {
            state.is_pressed()
        } else {
            false
        }
    }
    pub fn is_mouse_button_pressed(&self, mouse_button: MouseButton) -> bool {
        if let Some(state) = self.mouse_buttons.get(&mouse_button) {
            state.is_pressed()
        } else {
            false
        }
    }

    pub fn set_mouse_position(&mut self, window: &Window, position: Vec2<f32>) -> EngineResult {
        self.mouse_position_set = true;
        window.set_cursor_position(PhysicalPosition::new(position.x, position.y))?;
        Ok(())
    }
    pub fn mouse_visible(&self, window: &Window, visible: bool) {
        window.set_cursor_visible(visible);
    }
    pub fn mouse_grab(&self, window: &Window, mode: CursorGrabMode) -> EngineResult {
        window.set_cursor_grab(mode)?;
        Ok(())
    }
    pub fn mouse_position(&self) -> Vec2<f32> {
        self.last_mouse_position
    }
    /// How much the mouse has moved since the last frame
    pub fn delta_mouse(&self) -> Vec2<f32> {
        self.delta_mouse
    }
    /// Call every frame, `enabled` is there so that you can set a condition to set it free.
    /// Useful for fps style looking around.
    /// The mouse is kept in a circle with a radius of 1/4 the windows shortest length
    pub fn lock_mouse_near_center(&mut self, window: &Window, enabled: bool) {
        if !enabled {
            window.set_cursor_visible(true);
            return;
        }
        window.set_cursor_visible(false);
        let window_size = window.inner_size();
        let window_center = Vec2::new(
            window_size.width as f32 / 2.0,
            window_size.height as f32 / 2.0,
        );
        if (self.mouse_position() - window_center).magnitude()
            > window_size.width.min(window_size.height) as f32 / 4.0
        {
            self.set_mouse_position(window, window_center).unwrap();
        }
    }

    /// Must be called every window event
    pub(crate) fn process_input(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput { event, .. } = event {
            if let PhysicalKey::Code(key_code) = event.physical_key {
                self.keys.insert(key_code, event.state);
            }
        }
        if let WindowEvent::MouseInput { state, button, .. } = event {
            self.mouse_buttons.insert(*button, *state);
        }
        if let WindowEvent::CursorMoved { position, .. } = event {
            let mouse_position = Vec2::new(position.x as f32, position.y as f32);
            if self.mouse_position_set {
                self.last_mouse_position = mouse_position;
                self.delta_mouse = Vec2::zero();
                self.mouse_position_set = false;
            } else {
                self.delta_mouse = self.last_mouse_position - mouse_position;
                self.last_mouse_position = mouse_position;
            }
        }
    }

    /// Used to set delta_mouse to (0.0, 0.0) since it will stay at whatever the previous mouse movement was if the mouse isn't moving
    /// Should be called at the end of a `draw` so that the value is read before being reset
    pub(crate) fn reset(&mut self) {
        self.delta_mouse = Vec2::zero();
    }
}
