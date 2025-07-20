use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::SampleFormat;
use std::sync::{Arc, Mutex};
use crate::synth::Synthesizer;

pub struct AudioOutput {
    stream: Option<cpal::Stream>,
    synth: Arc<Mutex<Synthesizer>>,
}

impl AudioOutput {
    pub fn new(synth: Arc<Mutex<Synthesizer>>) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            stream: None,
            synth,
        })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or("No output device found")?;

        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;

        let synth_clone = Arc::clone(&self.synth);
        
        let stream = match config.sample_format() {
            SampleFormat::F32 => {
                device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        let mut synth = synth_clone.lock().unwrap();
                        for sample in data.iter_mut() {
                            *sample = synth.next_sample();
                        }
                    },
                    |err| eprintln!("Audio error: {}", err),
                    None,
                )?
            }
            SampleFormat::I16 => {
                device.build_output_stream(
                    &config.into(),
                    move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                        let mut synth = synth_clone.lock().unwrap();
                        for sample in data.iter_mut() {
                            let float_sample = synth.next_sample();
                            *sample = (float_sample * i16::MAX as f32) as i16;
                        }
                    },
                    |err| eprintln!("Audio error: {}", err),
                    None,
                )?
            }
            SampleFormat::U16 => {
                device.build_output_stream(
                    &config.into(),
                    move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                        let mut synth = synth_clone.lock().unwrap();
                        for sample in data.iter_mut() {
                            let float_sample = synth.next_sample();
                            *sample = ((float_sample + 1.0) * 0.5 * u16::MAX as f32) as u16;
                        }
                    },
                    |err| eprintln!("Audio error: {}", err),
                    None,
                )?
            }
            _ => {
                return Err("Unsupported sample format".into());
            }
        };

        stream.play()?;
        self.stream = Some(stream);
        
        println!("🎵 Audio output started at {} Hz", sample_rate);
        Ok(())
    }

    pub fn stop(&mut self) {
        self.stream = None;
        println!("🔇 Audio output stopped");
    }
} 