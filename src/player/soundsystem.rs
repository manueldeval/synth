
use cpal::traits::DeviceTrait;
use cpal::traits::EventLoopTrait;
use cpal::traits::HostTrait;

use std::sync::Arc;
use std::sync::Mutex;

use std::thread;
use std::thread::JoinHandle;

// use crate::synth::engine::synth::Synth;
// use crate::synth::engine::simple::SimpleSynth;
// use std::sync::mpsc::sync_channel;
// use std::sync::mpsc::Receiver;
use crossbeam::crossbeam_channel::Receiver;

struct SoundSystemInternals {
    sample_rate: i32,
    cpal_stream_id: cpal::StreamId,
    cpal_event_loop: cpal::EventLoop,
    cpal_channels: u16
}

pub struct SoundSystem {
    internals: Arc<Mutex<SoundSystemInternals>>
}

impl SoundSystem {
    pub fn build() -> SoundSystem { 
        let host = cpal::default_host();
        let device = host.default_output_device().expect("Failed to find a default output device.");
        let format = device.default_output_format().expect("Failed to find the audio format for device.");
        let channels = format.channels;
        let event_loop = host.event_loop();
        let stream_id = event_loop.build_output_stream(&device, &format).expect("Unable to create the audio stream.");
        SoundSystem {
            internals: Arc::new(Mutex::new(SoundSystemInternals{ sample_rate: format.sample_rate.0 as i32, cpal_stream_id: stream_id, cpal_event_loop:event_loop, cpal_channels: channels }))
        }
    }
    pub fn sample_rate(&self) -> i32 {
        self.internals.clone().lock().unwrap().sample_rate
    }

    pub fn start(&mut self, rx: Receiver<f32>) -> JoinHandle<()> {

        let internal_mutex_cloned = self.internals.clone();
        
        thread::spawn(move || {
            let internal_locked = internal_mutex_cloned.lock().unwrap(); 
            internal_locked.cpal_event_loop.play_stream(internal_locked.cpal_stream_id.clone()).expect("Unable to create stream!");
            let channels: usize = internal_locked.cpal_channels as usize;
   
            // Wait for producer!
            let _ = rx.recv().unwrap();

            internal_locked.cpal_event_loop.run(move |id, result| {
   
                let data = match result {
                    Ok(data) => data,
                    Err(err) => {
                        eprintln!("an error occurred on stream {:?}: {}", id, err);
                        return;
                    }
                };

                match data {
                    cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                        for sample in buffer.chunks_mut(channels) {
                            let received_value = rx.recv().unwrap();
                            let value = ((received_value * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                            for out in sample.iter_mut() {
                                *out = value;
                            }
                        }
                    },
                    cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                        for sample in buffer.chunks_mut(channels) {
                            let received_value = rx.recv().unwrap();
                            let value = (received_value * std::i16::MAX as f32) as i16;
                            for out in sample.iter_mut() {
                                *out = value;
                            }
                        }
                    },
                    cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                        for sample in buffer.chunks_mut(channels) {
                            let received_value = rx.recv().unwrap();
                            let value = received_value;
                            for out in sample.iter_mut() {
                                *out = value;
                            }
                        }
                    },
                    _ => (),
                }
            });
        })
    }

}
