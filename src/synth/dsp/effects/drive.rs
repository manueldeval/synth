use crate::synth::dsp::audionode::AudioNode;
use crate::synth::utils::converters::non_linearized;

/*
=========================================
Drive
=========================================
*/

pub struct DriveNode {
  input: f32,
  output: f32,
  amount: f32
}

impl DriveNode {
  pub const INPUT_VALUE: i32 = 0;
  pub const AMOUNT: i32 = 1;
  pub const OUTPUT_VALUE: i32 = 0;

  pub fn new() -> DriveNode {
    DriveNode { input: 0.0, output: 0.0, amount: 0.0  }
  }
}

impl AudioNode for DriveNode { 
  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      DriveNode::INPUT_VALUE => self.input = value,
      DriveNode::AMOUNT =>      self.amount = value,
      _ => ()
    };
  }
  fn compute(&mut self) {
    self.output = non_linearized(self.input,self.amount);
  }
  fn get_output_value(&self, ouput: i32) -> f32 { 
    match ouput {
      DriveNode::OUTPUT_VALUE => self.output,
      _ => 0.0
    }
  }
}