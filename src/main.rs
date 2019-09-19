#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_imports)]
//https://github.com/klingtnet/rosc

mod synth;
mod player;
mod osc;

extern crate cpal;
extern crate failure;
extern crate piston_window;

use std::sync::{Arc, Mutex};

use osc::osc::OSC;
use synth::synth::Synth;
use player::soundsystem::SoundSystem;

fn main() {
    println!("Starting osc receiver.");
    let osc = OSC::new(String::from("127.0.0.1"),6666);
    osc.start();

    println!("Starting sound system.");
    let mut sound_system = SoundSystem::build();
    let synth = Arc::new(Mutex::new(Synth::new(sound_system.sample_rate(), osc.receiver_factory())));
    
    println!("Starting sound system.");
    let sound_thread = sound_system.start(synth);
    println!("Started.");
    
    let _ = sound_thread.join();
    println!("Stopped.");

}

