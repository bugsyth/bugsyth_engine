use std::io;

use cpal::{
    BuildStreamError, DefaultStreamConfigError, DeviceNameError, DevicesError, PlayStreamError,
};
use glium::{
    DrawError, ProgramCreationError,
    framebuffer::{RenderBufferCreationError, ValidationError},
    index,
    program::ProgramChooserCreationError,
    texture::TextureCreationError,
    vertex,
    winit::error::{EventLoopError, ExternalError},
};
use image::ImageError;
use obj::ObjError;
use wav_io::reader::DecodeError;

pub type EngineResult<T = ()> = Result<T, EngineError>;
#[derive(Debug)]
pub enum EngineError {
    Error(String),
    GliumError(String),
    ImageError(String),
    ObjError(String),
    GltfError(String),
    AudioError(String),
    IoError(String),
}
impl std::error::Error for EngineError {}
impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(unreachable_patterns)]
        match self {
            Self::Error(msg) => write!(f, "{}", msg),
            Self::GliumError(msg) => write!(f, "{}", msg),
            Self::ImageError(msg) => write!(f, "{}", msg),
            Self::ObjError(msg) => write!(f, "{}", msg),
            Self::GltfError(msg) => write!(f, "{}", msg),
            Self::AudioError(msg) => write!(f, "{}", msg),
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
impl From<ExternalError> for EngineError {
    fn from(value: ExternalError) -> Self {
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

// Gltf
impl From<gltf::Error> for EngineError {
    fn from(value: gltf::Error) -> Self {
        Self::GltfError(value.to_string())
    }
}

// std::io
impl From<io::Error> for EngineError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

// cpal
impl From<DevicesError> for EngineError {
    fn from(value: DevicesError) -> Self {
        Self::AudioError(value.to_string())
    }
}
impl From<DeviceNameError> for EngineError {
    fn from(value: DeviceNameError) -> Self {
        Self::AudioError(value.to_string())
    }
}
impl From<DefaultStreamConfigError> for EngineError {
    fn from(value: DefaultStreamConfigError) -> Self {
        Self::AudioError(value.to_string())
    }
}
impl From<BuildStreamError> for EngineError {
    fn from(value: BuildStreamError) -> Self {
        Self::AudioError(value.to_string())
    }
}
impl From<PlayStreamError> for EngineError {
    fn from(value: PlayStreamError) -> Self {
        Self::AudioError(value.to_string())
    }
}

// wav-io
impl From<DecodeError> for EngineError {
    fn from(value: DecodeError) -> Self {
        Self::AudioError(value.to_string())
    }
}

impl From<&str> for EngineError {
    fn from(value: &str) -> Self {
        Self::Error(value.to_string())
    }
}
