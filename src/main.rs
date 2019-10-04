#[allow(dead_code)]
// #[allow(unused_mut)]
// #[allow(unused_imports)]

mod synth;
mod player;
mod osc;
mod graph;
mod web;

extern crate cpal;

#[cfg(feature = "oscilloscope")]
extern crate failure;
#[cfg(feature = "oscilloscope")]
extern crate piston_window;

use osc::osc::OSC;
use synth::engine::editable::EditableSynth;
use synth::commands::systemcommand::SystemCommandHandler;
use player::soundsystem::SoundSystem;
use crossbeam::crossbeam_channel::bounded;

use synth::commands::patch::Patch;
use web::webserver::*;

fn start(patch:  &Patch) -> Result<(),String> {
    println!("Starting osc receiver.");
    let osc = OSC::new(String::from("127.0.0.1"),6666);
    osc.start();

    println!("Starting sound system.");
    let mut sound_system = SoundSystem::build();

    println!("Creating synthetizer.");
    let mut synth = EditableSynth::new(sound_system.sample_rate(), osc.receiver_factory());

    println!("Loading patch.");
    for command in patch.commands.iter() {
        synth.receive_command(command)?;
    }

    println!("Starting sound system with graph: {}",synth.graph);
    let (sender, receiver) = bounded(200);
    println!("Started!");
    
    sound_system.start(receiver);
    loop {
        sender.send(synth.compute())
            .map_err(|e| format!("{}",e))?;
    }
}

fn main() -> Result<(),String> {
    let _ = start_web_server().join();
    Ok(())
    // let patch = Patch::from_file(&String::from("/home/deman/projets/perso/rustic/synth/patches/patch1.yaml"))?;
    // start(&patch)
}


    // let command0 = SystemCommand::Reset;

    // let command1 = SystemCommand::Create { id: String::from("a"), node_type: AudioNodeRegistry::SIN };
    // let command2 = SystemCommand::Link { 
    //     src_node: String::from("a"), src_port: 0,
    //     dst_node: String::from("master"), dst_port: 0 
    // };
    // let mut patch: Patch = Patch::new();
    // patch.add_command(&command0);
    // patch.add_command(&command2);

    // println!("{}",patch.to_json());  

    // let patch = Patch::from_json_file(&String::from("/home/deman/projets/perso/rustic/synth/patches/patch1.json"))?;
    // println!("{}",patch.to_json());
    
    
    // Ok(())

    // let mut serialized = serde_json::to_string(&command2).unwrap();
    // println!("{}",serialized);
    // serialized = serde_json::to_string(&command1).unwrap();
    // println!("{}",serialized);

    // let deserialized: SystemCommand = serde_json::from_str(&serialized).unwrap();
    // println!("{}",deserialized);

    // match start() {
    //     Err(e) => eprintln!("{}",e),
    //     _ => ()
    // };