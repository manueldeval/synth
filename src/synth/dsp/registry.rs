use crate::synth::dsp::audionode::AudioNode;
use crate::osc::osc::OSCReceiverFactory;
use crate::synth::commands::config::*;
use crate::synth::dsp::node_factory::*;

use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Clone, Display, EnumIter,Hash, Eq, PartialEq)]
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

  pub fn get_nodes_config_spec() -> HashMap<AudioNodeRegistry,Vec<ConfigSpec>> {
    AudioNodeRegistry::node_types()
      .iter()
      .map(|x| (x.clone(),(*x).get_node_factory().config_spec()))
      .collect()
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_serialize_get_nodes_config_spec() {
    let map = AudioNodeRegistry::get_nodes_config_spec();
    println!("{}",serde_json::to_string(&map).unwrap());
  }
}