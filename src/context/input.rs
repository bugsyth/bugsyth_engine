use glium::winit::{
    event::{ElementState, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};
use std::collections::HashMap;
use vek::Vec2;

pub struct Input {
    keys: HashMap<KeyCode, ElementState>,
    mouse_buttons: HashMap<MouseButton, ElementState>,
    last_mouse_position: Vec2<f32>,
    delta_mouse: Vec2<f32>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
            mouse_buttons: HashMap::new(),
            last_mouse_position: Vec2::zero(),
            delta_mouse: Vec2::zero(),
        }
    }

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
    pub fn mouse_position(&self) -> Vec2<f32> {
        self.last_mouse_position
    }
    /// How much the mouse has moved since the last frame
    pub fn delta_mouse(&self) -> Vec2<f32> {
        self.delta_mouse
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
            let delta = self.last_mouse_position - mouse_position;
            self.last_mouse_position = mouse_position;
            self.delta_mouse = delta;
        }
    }

    /// Used to set delta_mouse to (0.0, 0.0) since it will stay at whatever the previous mouse movement was if the mouse isn't moving
    /// Should be called at the end of a draw so that the value is read before being reset
    pub(crate) fn reset(&mut self) {
        self.delta_mouse = Vec2::zero();
    }
}
