use std::io;

use glium::{
    framebuffer::{RenderBufferCreationError, ValidationError},
    index,
    program::ProgramChooserCreationError,
    texture::TextureCreationError,
    vertex,
    winit::error::EventLoopError,
    DrawError, ProgramCreationError,
};
use image::ImageError;
use obj::ObjError;

pub type EngineResult<T = ()> = Result<T, EngineError>;
#[derive(Debug)]
pub enum EngineError {
    GliumError(String),
    ImageError(String),
    ObjError(String),
    IoError(String),
}
impl std::error::Error for EngineError {}
impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        match self {
            Self::GliumError(msg) => write!(f, "{}", msg),
            Self::ImageError(msg) => write!(f, "{}", msg),
            Self::ObjError(msg) => write!(f, "{}", msg),
            Self::IoError(msg) => write!(f, "{}", msg),
            _ => write!(f, "Unknown Error"),
        }
    }
}

// Glium
impl From<EventLoopError> for EngineError {
    fn from(value: EventLoopError) -> Self {
        Self::GliumError(value.to_string())
    }
}
impl From<DrawError> for EngineError {
    fn from(value: DrawError) -> Self {
        Self::GliumError(value.to_string())
    }
}
impl From<ValidationError> for EngineError {
    fn from(value: ValidationError) -> Self {
        Self::GliumError(value.to_string())
    }
}
impl From<TextureCreationError> for EngineError {
    fn from(value: TextureCreationError) -> Self {
        Self::GliumError(value.to_string())
    }
}
impl From<ProgramCreationError> for EngineError {
    fn from(value: ProgramCreationError) -> Self {
        Self::GliumError(value.to_string())
    }
}
impl From<ProgramChooserCreationError> for EngineError {
    fn from(value: ProgramChooserCreationError) -> Self {
        Self::GliumError(value.to_string())
    }
}
impl From<vertex::BufferCreationError> for EngineError {
    fn from(value: vertex::BufferCreationError) -> Self {
        Self::GliumError(value.to_string())
    }
}
impl From<index::BufferCreationError> for EngineError {
    fn from(value: index::BufferCreationError) -> Self {
        Self::GliumError(value.to_string())
    }
}
impl From<RenderBufferCreationError> for EngineError {
    fn from(value: RenderBufferCreationError) -> Self {
        Self::GliumError(value.to_string())
    }
}

// Image
impl From<ImageError> for EngineError {
    fn from(value: ImageError) -> Self {
        Self::ImageError(value.to_string())
    }
}

// Obj
impl From<ObjError> for EngineError {
    fn from(value: ObjError) -> Self {
        Self::ObjError(value.to_string())
    }
}

// std::io
impl From<io::Error> for EngineError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}
