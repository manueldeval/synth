use crate::synth::dsp::audionode::*;
use crate::synth::commands::config::ConfigSpec;

use crate::synth::dsp::oscillators::baseoscillator::OscillatorMode;
use crate::synth::dsp::oscillators::simple::*;
use crate::synth::dsp::inputs::keyboad::*;
use crate::synth::dsp::various::identity::*;
use crate::osc::osc::OSCReceiverFactory;


pub trait AudioNodeFactory {
  fn create(&self,osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode>;
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { IOSpec::empty() }
}

//===============================================================
// Sin Node
//===============================================================


pub struct SinFactory;
impl AudioNodeFactory for SinFactory {
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SinNode::new(OscillatorMode::AUDIO, 0.5, 0.5, true)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { SinNode::get_config_spec() }
  fn io_spec(&self) -> IOSpec { SinNode::get_io_spec() }
}

//===============================================================
// LFO Sin Node
//===============================================================

pub struct SinLfoFactory;
impl AudioNodeFactory for SinLfoFactory {
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SinNode::new(OscillatorMode::LFO, 0.5, 0.5, true)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { SinNode::get_config_spec() }
  fn io_spec(&self) -> IOSpec { SinNode::get_io_spec() }
}

//===============================================================
// Square Node
//===============================================================

pub struct SquareFactory;
impl AudioNodeFactory for SquareFactory {
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SquareNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { SquareNode::get_config_spec() }
  fn io_spec(&self) -> IOSpec { SquareNode::get_io_spec() }
}

//===============================================================
// Lfo Square Node
//===============================================================

pub struct SquareLfoFactory;
impl AudioNodeFactory for SquareLfoFactory {
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SquareNode::new(OscillatorMode::LFO, 0.0, 0.5, true)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { SquareNode::get_config_spec() }
  fn io_spec(&self) -> IOSpec { SquareNode::get_io_spec() }
}

//===============================================================
// KeyBoard
//===============================================================

pub struct KeyboardFactory;
impl AudioNodeFactory for KeyboardFactory {
  fn create(&self,osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(KeyboardNode::new(osc_receiver_factory)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { KeyboardNode::get_config_spec() }
  fn io_spec(&self) -> IOSpec { KeyboardNode::get_io_spec() }
}

//===============================================================
// Identity
//===============================================================

pub struct IdentityFactory;
impl AudioNodeFactory for IdentityFactory {
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(IdentityNode::new()) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { IdentityNode::get_config_spec() }
  fn io_spec(&self) -> IOSpec { IdentityNode::get_io_spec() }
}
