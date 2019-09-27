#[allow(dead_code)]
#[allow(unused_mut)]
#[allow(unused_imports)]

mod synth;
mod player;
mod osc;
mod graph;

extern crate cpal;
extern crate failure;
extern crate piston_window;

use osc::osc::OSC;
use synth::engine::editable::EditableSynth;
use synth::commands::systemcommand::SystemCommand;
use synth::commands::systemcommand::SystemCommandHandler;
use synth::dsp::registry::AudioNodeRegistry;
use player::soundsystem::SoundSystem;
use crossbeam::crossbeam_channel::bounded;
use crate::synth::utils::note::*;
use crate::synth::dsp::units::*;

fn start() -> Result<(),String> {
    println!("Starting osc receiver.");
    let osc = OSC::new(String::from("127.0.0.1"),6666);
    osc.start();

    println!("Starting sound system.");
    let mut sound_system = SoundSystem::build();
    //let mut synth = SimpleSynth::new(sound_system.sample_rate(), osc.receiver_factory());
    let mut synth = EditableSynth::new(sound_system.sample_rate(), osc.receiver_factory());

    synth.receive_command(&SystemCommand::Create { id: String::from("a"), node_type: AudioNodeRegistry::SIN })?;

    synth.receive_command(&SystemCommand::Link { 
        src_node: String::from("a"), src_port: 0,
        dst_node: String::from("master"), dst_port: 0 
    })?;

    println!("Starting sound system with graph: {}",synth.graph);
    let (sender, receiver) = bounded(200);
    println!("Started!");
    
    sound_system.start(receiver);
    loop {
        sender.send(synth.compute());
    }
    println!("Stopped.");
}

fn main() {
    let command1 = SystemCommand::Create { id: String::from("a"), node_type: AudioNodeRegistry::SIN };
    let command2 = SystemCommand::Link { 
        src_node: String::from("a"), src_port: 0,
        dst_node: String::from("master"), dst_port: 0 
    };


    let mut serialized = serde_json::to_string(&command2).unwrap();
    println!("{}",serialized);
    serialized = serde_json::to_string(&command1).unwrap();
    println!("{}",serialized);

    let deserialized: SystemCommand = serde_json::from_str(&serialized).unwrap();
    println!("{}",deserialized);

    // match start() {
    //     Err(e) => eprintln!("{}",e),
    //     _ => ()
    // };
}

