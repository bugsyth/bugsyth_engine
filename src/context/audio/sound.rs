use crate::error::EngineResult;
use std::{fs::File, sync::Arc};
use wav_io::header::WavHeader;

#[derive(Debug, Clone)]
pub struct Sound {
    pub header: WavHeader,
    pub samples: Arc<Vec<f32>>,
}

impl Sound {
    pub fn new(path: impl Into<String>) -> EngineResult<Self> {
        let (header, samples) = wav_io::read_from_file(File::open(path.into())?)?;
        Ok(Self {
            header,
            samples: Arc::new(samples),
        })
    }
}
