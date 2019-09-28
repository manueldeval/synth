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
  SIN,
  SIN_LFO,
  SQUARE,
  SQUARE_LFO,
  KEYBOARD,
  INDENTITY
}



impl AudioNodeRegistry {
  pub fn create_node(&self, sample_rate: i32, osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> {
    SinNode::get_config_spec();
    match self {
      AudioNodeRegistry::SIN        => Box::new(SinNode::new(OscillatorMode::AUDIO, 0.5, 0.5, true)),
      AudioNodeRegistry::SIN_LFO    => Box::new(SinNode::new(OscillatorMode::LFO, 0.5, 0.5, true)),
      AudioNodeRegistry::SQUARE     => Box::new(SquareNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true)),
      AudioNodeRegistry::SQUARE_LFO => Box::new(SquareNode::new(OscillatorMode::LFO, 0.0, 0.5, true)),
      AudioNodeRegistry::KEYBOARD   => Box::new(KeyboardNode::new(osc_receiver_factory)),
      AudioNodeRegistry::INDENTITY  => Box::new(IdentityNode::new()),
    }
  }
}

impl fmt::Display for AudioNodeRegistry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        AudioNodeRegistry::SIN        => write!(f,"SIN"),
        AudioNodeRegistry::SIN_LFO    => write!(f,"SIN_LFO"),
        AudioNodeRegistry::SQUARE     => write!(f,"SQUARE"),
        AudioNodeRegistry::SQUARE_LFO => write!(f,"SQUARE_LFO"),
        AudioNodeRegistry::INDENTITY  => write!(f,"INDENTITY"),
        AudioNodeRegistry::KEYBOARD  => write!(f,"KEYBOARD"),
      }
    }
}