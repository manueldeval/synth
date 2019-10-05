use crate::synth::dsp::audionode::*;
use crate::synth::commands::config::ConfigSpec;

use crate::synth::dsp::oscillators::baseoscillator::OscillatorMode;
use crate::synth::dsp::oscillators::simple::*;
use crate::synth::dsp::inputs::keyboad::*;
use crate::synth::dsp::inputs::knob::*;
use crate::synth::dsp::various::identity::*;
use crate::osc::osc::OSCReceiverFactory;
use crate::synth::commands::config::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct NodeInfos  {
  pub classifier: String,
  pub config_spec: Vec<ConfigSpec>,
  pub io_spec: IOSpec
}

pub trait AudioNodeFactory {
  fn create(&self,osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode>;
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { IOSpec::empty() }
  fn classifier(&self) -> String;
  fn node_infos(&self) -> NodeInfos {
    NodeInfos { classifier: self.classifier(), config_spec: self.config_spec(), io_spec: self.io_spec() }
  }
}

//===============================================================
// Sin Node
//===============================================================
fn baseoscillator_input_spec() -> Vec<ConnectorSpec> { 
  vec!(
    ConnectorSpec::new(String::from("FREQ"), String::from("")),
    ConnectorSpec::new(String::from("AMP"), String::from("")),
    ConnectorSpec::new(String::from("TRIGGER"), String::from("")),
    ConnectorSpec::new(String::from("TRIGGER_SYNC_PHASE"), String::from(""))
  )
}

fn baseoscillator_output_spec() -> Vec<ConnectorSpec> { 
  vec!(
    ConnectorSpec::new(String::from("OSC"), String::from("")),
    ConnectorSpec::new(String::from("TRIGGER_SYNC_PHASE"), String::from(""))
  ) 
}

//===============================================================
// Sin Node
//===============================================================


pub struct SinFactory;
impl AudioNodeFactory for SinFactory {
  fn classifier(&self) -> String { String::from("oscillator/sin") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SinNode::new(OscillatorMode::AUDIO, 0.5, 0.5, true)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { 
    IOSpec { inputs: baseoscillator_input_spec() ,outputs: baseoscillator_output_spec() }
  }
}

//===============================================================
// LFO Sin Node
//===============================================================

pub struct SinLfoFactory;
impl AudioNodeFactory for SinLfoFactory {
  fn classifier(&self) -> String { String::from("lfo/sin") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SinNode::new(OscillatorMode::LFO, 0.5, 0.5, true)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { 
    IOSpec { inputs: baseoscillator_input_spec() ,outputs: baseoscillator_output_spec() }
  }
}

//===============================================================
// Square Node
//===============================================================

pub struct SquareFactory;
impl AudioNodeFactory for SquareFactory {
  fn classifier(&self) -> String { String::from("oscillator/square") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SquareNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { 
    // Inputs
    let mut inputs = baseoscillator_input_spec();
    inputs.push(ConnectorSpec::new(String::from("RATIO"), String::from("")));
    // Outputs
    let outputs = baseoscillator_output_spec();
    
    IOSpec { inputs, outputs }
  }
}

//===============================================================
// Lfo Square Node
//===============================================================

pub struct SquareLfoFactory;
impl AudioNodeFactory for SquareLfoFactory {
  fn classifier(&self) -> String { String::from("lfo/square") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SquareNode::new(OscillatorMode::LFO, 0.0, 0.5, true)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> {Vec::new() }
  fn io_spec(&self) -> IOSpec { 
    // Inputs
    let mut inputs = baseoscillator_input_spec();
    inputs.push(ConnectorSpec::new(String::from("RATIO"), String::from("")));
    // Outputs
    let outputs = baseoscillator_output_spec();
    
    IOSpec { inputs, outputs }
  }
}

//===============================================================
// KeyBoard
//===============================================================

pub struct KeyboardFactory;
impl AudioNodeFactory for KeyboardFactory {
  fn classifier(&self) -> String { String::from("input/keyboard") }
  fn create(&self,osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(KeyboardNode::new(osc_receiver_factory)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { 
    vec!(
      ConfigSpec::new(String::from("osc_channel"), ConfigType::StringType)
    )  
  }
  fn io_spec(&self) -> IOSpec { 
    let inputs = Vec::new();
    let outputs = vec!(
      ConnectorSpec::new(String::from("FREQUENCY"), String::from("")),
      ConnectorSpec::new(String::from("NOTE_ON"), String::from("")),
    );
    
    IOSpec { inputs, outputs }
  }
}

//===============================================================
// Identity
//===============================================================

pub struct IdentityFactory;
impl AudioNodeFactory for IdentityFactory {
  fn classifier(&self) -> String { String::from("output/master") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(IdentityNode::new()) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec {
    let inputs = vec!(
      ConnectorSpec::new(String::from("INPUT"), String::from(""))
    );
    let outputs = Vec::new();
    IOSpec { inputs, outputs }
  }
}

//===============================================================
// Knob
//===============================================================

pub struct KnobFactory;
impl AudioNodeFactory for KnobFactory {
  fn classifier(&self) -> String { String::from("input/knob") }
  fn create(&self,osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(KnobNode::new(osc_receiver_factory,0.0,String::from("/knob"))) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { 
    vec!(
      ConfigSpec::new(String::from("osc_channel"), ConfigType::StringType),
      ConfigSpec::new(String::from("value"), ConfigType::FloatType)
    )  
  }
  fn io_spec(&self) -> IOSpec { 
    let inputs = Vec::new();
    let outputs = vec!(
      ConnectorSpec::new(String::from("VALUE"), String::from(""))
  );
    
    IOSpec { inputs, outputs }
  }
}
