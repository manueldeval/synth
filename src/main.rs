#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_imports)]

mod synth;
mod player;
mod osc;

extern crate cpal;
extern crate failure;
extern crate piston_window;

use std::sync::{Arc, Mutex};

use osc::osc::OSC;
use synth::engine::simple::SimpleSynth;
use player::soundsystem::SoundSystem;


fn start() {
    println!("Starting osc receiver.");
    let osc = OSC::new(String::from("127.0.0.1"),6666);
    osc.start();

    println!("Starting sound system.");
    let mut sound_system = SoundSystem::build();
    let synth = Arc::new(Mutex::new(SimpleSynth::new(sound_system.sample_rate(), osc.receiver_factory())));
    
    println!("Starting sound system.");
    let sound_thread = sound_system.start(synth);
    println!("Started.");
    
    let _ = sound_thread.join();
    println!("Stopped.");
}

mod graph;
use crate::synth::dsp::registry::AudioNodeRegistry;
use crate::graph::graph::DspGraph;

fn main() {
    let mut g: DspGraph = DspGraph::new();

    let id1 = String::from("Sin1");
    let id2 = String::from("Sin2");

    g.add_audio_node(&id1, AudioNodeRegistry::SQUARE_LFO);
    g.add_audio_node(&id2, AudioNodeRegistry::SQUARE);

    g.add_link(&id1, 1, &id1, 2);
    match g.add_link(&id1, 0, &id2, 0) {
        Ok(()) => println!("Create: Ok"),
        Err(s) => println!("Erreur: {}",s.as_str())
    }
    // println!("{}",g);
    // match g.remove_node(&id1) {
    //     Ok(()) => println!("Remove: Ok"),
    //     Err(s) => println!("Erreur: {}",s.as_str())
    // }

    println!("{}",g);

    g.compute();
    // start();
}

