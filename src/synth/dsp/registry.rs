use crate::synth::dsp::audionode::AudioNode;
use crate::synth::dsp::oscillators::simple::SinNode;
use crate::synth::dsp::oscillators::simple::SquareNode;
use crate::synth::dsp::various::identity::IdentityNode;
use crate::synth::dsp::oscillators::baseoscillator::OscillatorMode;
use crate::osc::osc::OSCReceiverFactory;
use crate::synth::dsp::units::*;
use crate::synth::utils::note::*;

use std::fmt;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Clone)]
pub enum AudioNodeRegistry {
  SIN { freq: OptOscFreq, amp: OptAmp },
  SIN_LFO,
  SQUARE,
  SQUARE_LFO,
  INDENTITY
}



impl AudioNodeRegistry {
  pub fn create_node(&self, sample_rate: i32, osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> {
    match self {
      AudioNodeRegistry::SIN { freq, amp }  => Box::new(SinNode::new(OscillatorMode::AUDIO, 
        freq.clone().get_or_insert(OscFreq::Note(Note::A4)).volts(),
        amp.clone().get_or_insert(Amp::Val(0.5)).volts(), 
        true
      )),
      AudioNodeRegistry::SIN_LFO    => Box::new(SinNode::new(OscillatorMode::LFO, 0.5, 0.5, true)),
      AudioNodeRegistry::SQUARE     => Box::new(SquareNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true)),
      AudioNodeRegistry::SQUARE_LFO => Box::new(SquareNode::new(OscillatorMode::LFO, 0.0, 0.5, true)),
      AudioNodeRegistry::INDENTITY  => Box::new(IdentityNode::new())
    }
  }
}

impl fmt::Display for AudioNodeRegistry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        AudioNodeRegistry::SIN { freq, amp }  => write!(f,"SIN"),
        AudioNodeRegistry::SIN_LFO    => write!(f,"SIN_LFO"),
        AudioNodeRegistry::SQUARE     => write!(f,"SQUARE"),
        AudioNodeRegistry::SQUARE_LFO => write!(f,"SQUARE_LFO"),
        AudioNodeRegistry::INDENTITY  => write!(f,"INDENTITY"),
      }
    }
}