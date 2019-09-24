use crate::synth::dsp::audionode::AudioNode;
use crate::synth::utils::converters::voltage_to_zero_to_one;
use crate::synth::utils::converters::voltage_to_boolean;

use rand::prelude::*;

pub struct RandNode {
  value: f32,
  amp: f32,
  input_trigger: bool
}

impl RandNode {
  pub const INPUT_AMP: i32 = 0;
  pub const INPUT_TRIGGER: i32 = 1;

  pub const OUTPUT_SIGNAL: i32 = 0;

  pub fn new(amp: f32) -> RandNode {
    RandNode { value: 0.0, amp:amp , input_trigger: true }
  }
}

impl AudioNode for RandNode {
  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      RandNode::INPUT_AMP => self.amp = voltage_to_zero_to_one(value),
      RandNode::INPUT_TRIGGER => self.input_trigger = voltage_to_boolean(value),
      _ => ()  
    }
  }
  fn compute(&mut self) {
    let r : f32 = random();
    self.value = if self.input_trigger { (2.0 * r - 1.0) * self.amp } else { 0.0 }
  }
  fn get_output_value(&self, output: i32) -> f32 { 
    match output {
      RandNode::OUTPUT_SIGNAL => self.value ,
      _ => 0.0
    }
  }
}

