use crate::synth::dsp::audionode::*;
use crate::synth::commands::config::ConfigSpec;

use crate::synth::dsp::oscillators::baseoscillator::OscillatorMode;
use crate::synth::dsp::oscillators::simple::*;
use crate::synth::dsp::oscillators::rand::*;
use crate::synth::dsp::inputs::keyboad::*;
use crate::synth::dsp::inputs::knob::*;
use crate::synth::dsp::various::samplehold::*;
use crate::synth::dsp::various::mixer::*;
use crate::synth::dsp::filters::moog::*;
use crate::synth::dsp::effects::drive::*;

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
// Base oscillator helper
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
// Saw Node
//===============================================================

pub struct SawFactory;
impl AudioNodeFactory for SawFactory {
  fn classifier(&self) -> String { String::from("oscillator/saw") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SawNode::new(OscillatorMode::AUDIO, 0.0, 0.5, true,0.0)) } 
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
// Lfo Saw Node
//===============================================================

pub struct SawLfoFactory;
impl AudioNodeFactory for SawLfoFactory {
  fn classifier(&self) -> String { String::from("lfo/saw") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SawNode::new(OscillatorMode::LFO, 0.0, 0.5, true,0.0)) } 
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


//===============================================================
// Rand Node
//===============================================================

pub struct RandFactory;
impl AudioNodeFactory for RandFactory {
  fn classifier(&self) -> String { String::from("oscillator/rand") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(RandNode::new(0.0)) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { 
    // Inputs
    let inputs = vec!(
      ConnectorSpec::new(String::from("AMP"), String::from("")),
      ConnectorSpec::new(String::from("TRIGGER"), String::from(""))
    );
    // Outputs
    let outputs =   vec!(
      ConnectorSpec::new(String::from("OSC"), String::from("")),
    ); 
    IOSpec { inputs, outputs }
  }
}
//===============================================================
// Sample Hold Node
//===============================================================

pub struct SampleHoldFactory;
impl AudioNodeFactory for SampleHoldFactory {
  fn classifier(&self) -> String { String::from("various/sample_and_hold") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(SampleHoldNode::new()) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { 
    // Inputs
    let inputs = vec!(
      ConnectorSpec::new(String::from("TRIGGER"), String::from("")),
      ConnectorSpec::new(String::from("INPUT"), String::from(""))
    );
    // Outputs
    let outputs =   vec!(
      ConnectorSpec::new(String::from("OUTPUT"), String::from("")),
    ); 
    IOSpec { inputs, outputs }
  }
}

//===============================================================
// Moog filter
//===============================================================

pub struct MoogFilterFactory;
impl AudioNodeFactory for MoogFilterFactory {
  fn classifier(&self) -> String { String::from("filter/sample_and_hold") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(MoogFilterNode::new()) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { 
    // Inputs
    let inputs = vec!(
      ConnectorSpec::new(String::from("INPUT"), String::from("")),
      ConnectorSpec::new(String::from("CUTOFF"), String::from("")),
      ConnectorSpec::new(String::from("RESONANCE"), String::from("")),
    );
    // Outputs
    let outputs =   vec!(
      ConnectorSpec::new(String::from("OUTPUT LP"), String::from("")),
      ConnectorSpec::new(String::from("OUTPUT BP"), String::from("")),
      ConnectorSpec::new(String::from("OUTPUT HP"), String::from(""))
    ); 
    IOSpec { inputs, outputs }
  }
}


//===============================================================
// Mixer filter
//===============================================================

pub struct MixerFactory;
impl AudioNodeFactory for MixerFactory {
  fn classifier(&self) -> String { String::from("various/mixer") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(MixerNode::new()) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { 
    // Inputs
    let inputs = vec!(
      ConnectorSpec::new(String::from("IN 1"), String::from("")),
      ConnectorSpec::new(String::from("AMP 1"), String::from("")),
      ConnectorSpec::new(String::from("IN 2"), String::from("")),
      ConnectorSpec::new(String::from("AMP 2"), String::from("")),
      ConnectorSpec::new(String::from("IN 3"), String::from("")),
      ConnectorSpec::new(String::from("AMP 3"), String::from("")),
      ConnectorSpec::new(String::from("IN 4"), String::from("")),
      ConnectorSpec::new(String::from("AMP 4"), String::from("")),
      ConnectorSpec::new(String::from("OUTPUT AMP"), String::from("")),
    );
    // Outputs
    let outputs =   vec!(
      ConnectorSpec::new(String::from("OUTPUT"), String::from(""))
    ); 
    IOSpec { inputs, outputs }
  }
}

//===============================================================
// Mixer filter
//===============================================================

pub struct DriveFactory;
impl AudioNodeFactory for DriveFactory {
  fn classifier(&self) -> String { String::from("effect/drive") }
  fn create(&self,_osc_receiver_factory: &OSCReceiverFactory) -> Box<dyn AudioNode> { Box::new(DriveNode::new()) } 
  fn config_spec(&self) -> Vec<ConfigSpec> { Vec::new() }
  fn io_spec(&self) -> IOSpec { 
    // Inputs
    let inputs = vec!(
      ConnectorSpec::new(String::from("INPUT"), String::from("")),
      ConnectorSpec::new(String::from("AMOUNT"), String::from("")),
    );
    // Outputs
    let outputs =   vec!(
      ConnectorSpec::new(String::from("OUTPUT"), String::from(""))
    ); 
    IOSpec { inputs, outputs }
  }
}