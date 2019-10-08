#[allow(dead_code)]
// #[allow(unused_mut)]
// #[allow(unused_imports)]

mod synth;
mod player;
mod osc;
mod graph;
mod web;
mod config;
mod patch;

extern crate cpal;

#[cfg(feature = "oscilloscope")]
extern crate failure;
#[cfg(feature = "oscilloscope")]
extern crate piston_window;

use osc::osc::OSC;
use synth::engine::editable::*;
use player::soundsystem::SoundSystem;
use crossbeam::crossbeam_channel::bounded;
use synth::commands::controller::*;
use web::webserver::*;
use config::config::*;
use std::env;
use patch::manager::*;

fn main() -> Result<(),String> {
    let config_path = env::var("SYNTH_CONFIG").unwrap_or(String::from("./config.yml"));
    let config = load_config(config_path.as_str())?;
    println!("Config used '{}' : {:?}", config_path, config);
    
    // let patch_manager = PatchManager::new(config.patches_path.as_str());
    // println!("{:?}",patch_manager.patches()?);

    // let patch = patch_manager.load_patch("patch1")?;

    let (http_command_sender,  controller_command_receiver) = bounded(2000);
    let (controller_command_sender, synth_command_receiver) = bounded(2000);
    let (audio_sender, audio_receiver) = bounded(200);

    let webserver = Webserver::new(&config.web_ip,config.web_port,http_command_sender);
    let command_controller_thread = CommandControllerThread::new(controller_command_receiver, controller_command_sender);
    let osc = OSC::new(&config.osc_ip,config.osc_port);
    let mut sound_system = SoundSystem::build();
    let synth_thread = EditableSynthThread::new(sound_system.sample_rate(), osc.receiver_factory(),audio_sender,synth_command_receiver);

    println!("Starting synth thread.");
    let synth_join = synth_thread.start();
    println!("Starting sound system thread.");
    let sound_join = sound_system.start(audio_receiver);
    println!("Starting osc.");
    let osc_join = osc.start();
    println!("Starting web interface.");
    let webserver_join = webserver.start();
    println!("Starting command controller.");
    let command_controller_join = command_controller_thread.start(); 
    println!("System started!");

    let _ = synth_join.join();
    let _ = sound_join.join();
    let _ = osc_join.join();
    let _ = webserver_join.join();
    let _ = command_controller_join.join();
    
    Ok(())
}
