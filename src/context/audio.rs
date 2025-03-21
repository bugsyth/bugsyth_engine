use crate::error::{EngineError, EngineResult};
use cpal::{
    Device, FromSample, Host, OutputCallbackInfo, Sample, SupportedStreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use sound::Sound;
use std::{collections::HashMap, sync::Arc, thread, time::Duration};

pub mod sound;

pub mod audio_play_value {
    use std::sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    };
    /// Used in conjunction with `atomic_f64_load` and `atomic_f64_store`
    pub type AudioPlayValue = Arc<AtomicU64>;

    pub fn new_audio_play_value(value: f64) -> AudioPlayValue {
        let arc = Arc::new(AtomicU64::default());
        atomic_f64_store(&arc, value);
        arc
    }

    pub fn atomic_f64_load(atomic: &AudioPlayValue) -> f64 {
        f64::from_bits(atomic.load(Ordering::Relaxed))
    }

    pub fn atomic_f64_store(atomic: &AudioPlayValue, value: f64) {
        atomic.store(value.to_bits(), Ordering::Relaxed);
    }
}

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

    /// There is noise when the file is done but the thread is still active will try to fix sometime. Volume and speed can be changed using values of type
    /// `Arc<AtomicU64>` that are wrapped in `AudioPlayValue` you can use `atomic_f64_store` and `atomic_f64_load` to manage that data.
    pub fn play(
        &mut self,
        sound: &Sound,
        volume: &audio_play_value::AudioPlayValue,
        speed: &audio_play_value::AudioPlayValue,
    ) -> EngineResult {
        let (device, config) = self.output_devices.get(&self.active_output_device).unwrap();
        let (device, config) = (device.clone(), config.clone());
        let samples = Arc::clone(&sound.samples);
        let sound_sample_rate = sound.header.sample_rate as f64;
        let sound_channels = sound.header.channels;
        let volume = volume.clone();
        let speed = speed.clone();

        rayon::spawn(move || {
            let sample_rate = config.sample_rate().0 as f64;
            let sample_rate_ratio = sound_sample_rate / sample_rate;
            let duration = samples.len() as f64 / sample_rate_ratio;
            let channels = config.channels() as usize;

            let mut sample_clock = 0.0f64;

            let mut next_value = move || {
                let volume = audio_play_value::atomic_f64_load(&volume);
                let speed = audio_play_value::atomic_f64_load(&speed);
                let index = (sample_clock.floor() as usize) * sound_channels as usize;
                let next_index = ((sample_clock.floor() as usize) + 1)
                    .min(samples.len() / sound_channels as usize - 1)
                    * sound_channels as usize;

                let frac = sample_clock - sample_clock.floor();
                let mut output_sample = 0.0;

                for ch in 0..sound_channels as usize {
                    let sample1 = samples.get(index + ch).copied().unwrap_or(0.0) as f64;
                    let sample2 = samples.get(next_index + ch).copied().unwrap_or(0.0) as f64;
                    output_sample += (1.0 - frac) * sample1 + frac * sample2; // Linear interpolation per channel
                }

                sample_clock += sample_rate_ratio * speed;
                (output_sample * volume / sound_channels as f64) as f32 // Normalize volume across channels
            };

            let err_fn = |err| eprintln!("An error occurred on stream: {}", err);

            let stream = device
                .build_output_stream(
                    &SupportedStreamConfig::into(config.clone()),
                    move |data: &mut [f32], _: &OutputCallbackInfo| {
                        write_data(data, channels, &mut next_value);
                    },
                    err_fn,
                    None,
                )
                .unwrap();

            stream.play().unwrap();
            // Let the whole clip play out
            thread::sleep(Duration::from_secs_f64(duration));
        });
        Ok(())
    }

    pub fn get_output_device_names(&self) -> Vec<String> {
        self.output_devices.keys().cloned().collect()
    }

    pub fn set_output_device(&mut self, name: String) -> EngineResult {
        if self.output_devices.contains_key(&name) {
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
        match self.host.default_output_device() {
            Some(device) => {
                self.active_output_device = device.name()?;
            }
            _ => {
                return Err(EngineError::AudioError("No default device".to_string()));
            }
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
