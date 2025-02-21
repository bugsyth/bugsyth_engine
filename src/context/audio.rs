use crate::error::{EngineError, EngineResult};
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, Host, OutputCallbackInfo, Sample, SupportedStreamConfig,
};
use sound::Sound;
use std::{collections::HashMap, sync::Arc, thread, time::Duration};

pub mod sound;

pub struct Audio {
    host: Host,
    output_devices: HashMap<String, (Device, SupportedStreamConfig)>,
    active_output_device: String,
}

impl Audio {
    pub(crate) fn new() -> EngineResult<Self> {
        // Might need to add host to Audio later
        let host = cpal::default_host();
        let mut output_devices = HashMap::new();
        for device in host.output_devices()? {
            let config = device.default_output_config()?;
            output_devices.insert(device.name()?, (device, config));
        }
        let mut active_output_device = String::new();
        if let Some(device) = host.default_output_device() {
            active_output_device = device.name()?;
        }
        Ok(Self {
            host,
            output_devices,
            active_output_device,
        })
    }

    pub fn play(&mut self, sound: &Sound) -> EngineResult {
        let (device, config) = self.output_devices.get(&self.active_output_device).unwrap();
        let (device, config) = (device.clone(), config.clone());
        let samples = Arc::clone(&sound.samples);
        rayon::spawn(move || {
            let sample_rate = config.sample_rate().0 as f32;
            let duration = samples.len() as f32 / sample_rate;
            let channels = config.channels() as usize;

            let mut sample_clock = 0u64;
            let mut next_value = move || {
                sample_clock += 1;
                if let Some(sample) = samples.get(sample_clock as usize) {
                    *sample
                } else {
                    0.0
                }
            };

            let err_fn = |err| eprintln!("An error occured on stream: {}", err);

            let stream = Arc::new(
                device
                    .build_output_stream(
                        &SupportedStreamConfig::into(config.clone()),
                        move |data: &mut [f32], _: &OutputCallbackInfo| {
                            write_data(data, channels, &mut next_value);
                        },
                        err_fn,
                        None,
                    )
                    .unwrap(),
            );

            stream.play().unwrap();
            thread::sleep(Duration::from_secs_f32(duration));
        });
        Ok(())
    }

    pub fn get_output_device_names(&self) -> Vec<String> {
        self.output_devices.keys().map(|key| key.clone()).collect()
    }

    pub fn set_output_device(&mut self, name: String) -> EngineResult {
        if let Some(_) = self.output_devices.get(&name) {
            self.active_output_device = name;
        } else {
            return Err(EngineError::AudioError(format!(
                "Can't find device: {}",
                name
            )));
        }
        Ok(())
    }
    pub fn set_output_device_as_default_device(&mut self) -> EngineResult {
        if let Some(device) = self.host.default_output_device() {
            self.active_output_device = device.name()?;
        } else {
            return Err(EngineError::AudioError("No default device".to_string()));
        }
        Ok(())
    }
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
