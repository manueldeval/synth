use crate::synth::dsp::audionode::AudioNode;
use crate::synth::dsp::oscillators::simple::SinNode;
use crate::synth::dsp::oscillators::simple::SquareNode;
use crate::synth::dsp::various::identity::IdentityNode;
use crate::synth::dsp::oscillators::baseoscillator::OscillatorMode;
use crate::synth::dsp::inputs::keyboad::KeyboardNode;
use crate::osc::osc::OSCReceiverFactory;
use crate::synth::dsp::units::*;
use crate::synth::utils::note::*;
use crate::synth::commands::config::*;
use crate::synth::dsp::node_factory::*;

use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;

use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Clone, Display, EnumIter)]
pub enum AudioNodeRegistry {
  Sin,
  SinLfo,
  Square,
  SquareLfo,
  Keyboard,
  Identity
}

impl AudioNodeRegistry {
  pub fn get_node_factory(&self) -> Box<dyn AudioNodeFactory> {
    match self {
      AudioNodeRegistry::Sin       => Box::new(SinFactory),
      AudioNodeRegistry::SinLfo    => Box::new(SinLfoFactory),
      AudioNodeRegistry::Square    => Box::new(SquareFactory),
      AudioNodeRegistry::SquareLfo => Box::new(SquareLfoFactory),
      AudioNodeRegistry::Keyboard  => Box::new(KeyboardFactory),
      AudioNodeRegistry::Identity  => Box::new(IdentityFactory)
    }
  }

  pub fn node_types() -> Vec<AudioNodeRegistry> {
    AudioNodeRegistry::iter().collect()
  }

  pub fn create_node(&self,sample_rate: i32, osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> {
    let mut node = self.get_node_factory().create(osc_receiver_factory);
    node.set_sample_rate(sample_rate);
    node
  }

}

