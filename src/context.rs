use crate::error::EngineResult;
use audio::Audio;
use camera::CameraState;
use glium::{Display, Program, glutin::surface::WindowSurface, winit::window::Window};
use input::Input;
use std::{collections::HashMap, f32::consts::PI};
use vek::Vec3;

pub mod audio;
pub mod camera;
mod input;

/// Holds everything that the user can use for event handling, audio, and basic boilerplate
pub struct Context {
    pub window: Window,
    pub display: Display<WindowSurface>,
    pub input: Input,
    pub audio: Audio,
    pub camera: CameraState,
    pub dt: f32,
    pub fixed_update: FixedUpdate,
    programs: HashMap<String, Program>,
}

impl Context {
    pub(crate) fn new(window: Window, display: Display<WindowSurface>) -> EngineResult<Self> {
        let window_size = window.inner_size();
        Ok(Self {
            window,
            display,
            input: Input::default(),
            audio: Audio::new()?,
            camera: CameraState::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                PI / 2.0,
                window_size.width as f32 / window_size.height as f32,
                0.001,
                1000.0,
            ),
            dt: 0.0,
            fixed_update: FixedUpdate {
                accumulator: 0.0,
                tick_rate: 0.0166,
            },
            programs: HashMap::new(),
        })
    }

    /// Same as `add_program` but creates the program for you
    pub fn new_program(
        &mut self,
        name: impl Into<String>,
        vert: &str,
        frag: &str,
        geom: Option<&str>,
    ) -> EngineResult {
        self.programs.insert(
            name.into(),
            Program::from_source(&self.display, vert, frag, geom)?,
        );
        Ok(())
    }

    pub fn add_program(&mut self, name: impl Into<String>, program: Program) -> Option<Program> {
        self.programs.insert(name.into(), program)
    }
    pub fn remove_program(&mut self, name: impl Into<String>) -> Option<Program> {
        self.programs.remove(&name.into())
    }

    pub fn get_program(&self, name: impl Into<String>) -> Option<&Program> {
        self.programs.get(&name.into())
    }
    pub fn get_program_mut(&mut self, name: impl Into<String>) -> Option<&mut Program> {
        self.programs.get_mut(&name.into())
    }
}

/// Change tick_rate to what you want it to be in your game
/// Default is 0.0166 or 60 fps
pub struct FixedUpdate {
    pub(crate) accumulator: f32,
    pub tick_rate: f32,
}
