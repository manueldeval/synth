#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_imports)]
//https://github.com/klingtnet/rosc
mod synth;
mod osc;
use osc::osc::{OSC,OSCReceiverFactory};
use synth::synth::Synth;
extern crate cpal;
extern crate failure;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

extern crate piston_window;


fn runsynth(receiver_factory: OSCReceiverFactory) -> Result<(), failure::Error> {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("failed to find a default output device");
    let format = device.default_output_format()?;
    let event_loop = host.event_loop();
    let stream_id = event_loop.build_output_stream(&device, &format)?;
    event_loop.play_stream(stream_id.clone())?;

    let mut synth = Synth::new(format.sample_rate.0 as i32, receiver_factory);


    event_loop.run(move |id, result| {

        let data = match result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", id, err);
                return;
            }
        };

        match data {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = ((synth.compute() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (synth.compute() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = synth.compute();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            _ => (),
        }

    });
}




fn main() {
    let osc = OSC::new(String::from("127.0.0.1"),6666);
    osc.start();
    runsynth(osc.receiver_factory());
    // oscHandler.join();
}

