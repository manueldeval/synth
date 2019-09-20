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
use synth::synth::Synth;
use player::soundsystem::SoundSystem;


fn start() {
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

mod graph;
use graph::graph::Graph;
use crate::synth::factory::AUDIO_NODE_TYPE;

fn main() {
    // let mut g: Graph = Graph::new();
    // for i in (0..5) {
    //     g.add_audio_node(&String::from(format!("SinA{}",i)), AUDIO_NODE_TYPE::SQUARE);
    //     g.add_audio_node(&String::from(format!("SinB{}",i)), AUDIO_NODE_TYPE::SQUARE);
    //     g.add_link(&String::from(format!("SinA{}",i)), 0,&String::from(format!("SinB{}",i)),0);
    // }

    // // match g.add_link(&String::from("Sin1"), 0,&String::from("Sin2"),1) {
    // //     Ok(()) => println!("Ok"),
    // //     Err(s) => println!("Erreur: {}",s.as_str())
    // // }
    
    // for i in 0..60*44_000*60 {
    //     g.compute();
    // }
    start();
}

