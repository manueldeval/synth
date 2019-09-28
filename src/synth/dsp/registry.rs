use crate::synth::dsp::audionode::AudioNode;
use crate::synth::dsp::oscillators::simple::SinNode;
use crate::synth::dsp::oscillators::simple::SquareNode;
use crate::synth::dsp::various::identity::IdentityNode;
use crate::synth::dsp::oscillators::baseoscillator::OscillatorMode;
use crate::synth::dsp::inputs::keyboad::KeyboardNode;
use crate::osc::osc::OSCReceiverFactory;
use crate::synth::dsp::units::*;
use crate::synth::utils::note::*;

use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Clone)]
pub enum AudioNodeRegistry {
  Sin,
  SinLfo,
  Square,
  SquareLfo,
  Keyboard,
  Identity
}



impl AudioNodeRegistry {
  pub fn create_node(&self, sample_rate: i32, osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> {
    SinNode::get_config_spec();
    match self {
      AudioNodeRegistry::Sin       => Box::new(SinNode::new(OscillatorMode::AUDIO, 0.5, 0.5, true)),
      AudioNodeRegistry::SinLfo    => Box::new(SinNode::new(OscillatorMode::LFO, 0.5, 0.5, true)),
      AudioNodeRegistry::Square    => Box::new(SquareNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true)),
      AudioNodeRegistry::SquareLfo => Box::new(SquareNode::new(OscillatorMode::LFO, 0.0, 0.5, true)),
      AudioNodeRegistry::Keyboard   => Box::new(KeyboardNode::new(osc_receiver_factory)),
      AudioNodeRegistry::Identity  => Box::new(IdentityNode::new()),
    }
  }
}

impl fmt::Display for AudioNodeRegistry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        AudioNodeRegistry::Sin       => write!(f,"SIN"),
        AudioNodeRegistry::SinLfo    => write!(f,"SIN_LFO"),
        AudioNodeRegistry::Square    => write!(f,"SQUARE"),
        AudioNodeRegistry::SquareLfo => write!(f,"SQUARE_LFO"),
        AudioNodeRegistry::Keyboard   => write!(f,"INDENTITY"),
        AudioNodeRegistry::Identity  => write!(f,"KEYBOARD"),
      }
    }
}