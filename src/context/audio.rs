use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, OutputCallbackInfo, Sample, SizedSample, StreamConfig,
    SupportedStreamConfig,
};
use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub struct Audio {
    device: Device,
    config: SupportedStreamConfig,
}

impl Audio {
    pub fn new() -> Self {
        // Might need to add host to Audio later
        let host = cpal::default_host();
        let device = host.default_output_device().unwrap();
        let config = device.default_output_config().unwrap();

        Self { device, config }
    }

    pub fn noise_test(&self) {
        match self.config.sample_format() {
            cpal::SampleFormat::I8 => run::<i8>(&self.device, &self.config.clone().into()),
            cpal::SampleFormat::I16 => run::<i16>(&self.device, &self.config.clone().into()),
            // cpal::SampleFormat::I24 => run::<I24>(&self.device, &self.config.clone().into()),
            cpal::SampleFormat::I32 => run::<i32>(&self.device, &self.config.clone().into()),
            // cpal::SampleFormat::I48 => run::<I48>(&self.device, &self.config.clone().into()),
            cpal::SampleFormat::I64 => run::<i64>(&self.device, &self.config.clone().into()),
            cpal::SampleFormat::U8 => run::<u8>(&self.device, &self.config.clone().into()),
            cpal::SampleFormat::U16 => run::<u16>(&self.device, &self.config.clone().into()),
            // cpal::SampleFormat::U24 => run::<U24>(&self.device, &self.config.clone().into()),
            cpal::SampleFormat::U32 => run::<u32>(&self.device, &self.config.clone().into()),
            // cpal::SampleFormat::U48 => run::<U48>(&self.device, &self.config.clone().into()),
            cpal::SampleFormat::U64 => run::<u64>(&self.device, &self.config.clone().into()),
            cpal::SampleFormat::F32 => run::<f32>(&self.device, &self.config.clone().into()),
            cpal::SampleFormat::F64 => run::<f64>(&self.device, &self.config.clone().into()),
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        }
    }
}

fn run<T>(device: &Device, config: &StreamConfig)
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let is_playing = Arc::new(Mutex::new(true));
    let is_playing_clone = Arc::clone(&is_playing);

    let device = device.clone();
    let config = config.clone();

    let handle = thread::spawn(move || {
        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 440.0 * 2.0 * PI / sample_rate).sin()
        };

        let err_fn = |err| eprintln!("Error ovvurred on stream: {}", err);

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [T], _: &OutputCallbackInfo| {
                    write_data(data, channels, &mut next_value);
                },
                err_fn,
                None,
            )
            .unwrap();
        stream.play().unwrap();

        while *is_playing_clone.lock().unwrap() {
            thread::sleep(Duration::from_millis(100));
        }
    });

    *is_playing.lock().unwrap() = false;
    handle.join().unwrap();
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample_(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
