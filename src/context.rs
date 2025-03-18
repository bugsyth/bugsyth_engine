use crate::{
    error::EngineResult,
    shaders::{TEXT_FS, TEXT_VS},
};
use audio::Audio;
use camera::CameraState;
use font::Font;
use glium::{Display, Program, glutin::surface::WindowSurface, winit::window::Window};
use input::Input;
use std::{collections::HashMap, f32::consts::PI};
use vek::Vec3;

pub mod audio;
pub mod camera;
pub(crate) mod font;
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
    fonts: HashMap<String, Font>,
}

impl Context {
    pub(crate) fn new(window: Window, display: Display<WindowSurface>) -> EngineResult<Self> {
        let window_size = window.inner_size();

        let mut programs = HashMap::new();
        programs.insert(
            "text".to_string(),
            Program::from_source(&display, TEXT_VS, TEXT_FS, None)?,
        );

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
            programs,
            fonts: HashMap::new(),
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

    /// Do not add a program called "text" as it is used for text
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

    pub fn add_font(
        &mut self,
        name: impl Into<String>,
        font: &[u8],
        font_size: f32,
    ) -> EngineResult {
        self.fonts
            .insert(name.into(), Font::new(&self.display, font, font_size)?);
        Ok(())
    }
    pub fn remove_font(&mut self, name: impl Into<String>) {
        self.fonts.remove(&name.into());
    }

    pub(crate) fn get_font(&self, name: impl Into<String>) -> Option<&Font> {
        self.fonts.get(&name.into())
    }
    // pub(crate) fn get_font_mut(&mut self, name: impl Into<String>) -> Option<&mut Font> {
    //     self.fonts.get_mut(&name.into())
    // }
}

/// Change tick_rate to what you want it to be in your game
/// Default is 0.0166 or 60 fps
pub struct FixedUpdate {
    pub(crate) accumulator: f32,
    pub tick_rate: f32,
}
