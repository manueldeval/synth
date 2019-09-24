use crate::synth::dsp::audionode::AudioNode;

/*
=========================================
Null
=========================================
*/

pub struct IdentityNode {
  value: f32
}

impl IdentityNode {
  pub const INPUT_VALUE: i32 = 0;
  pub const OUTPUT_VALUE: i32 = 0;

  pub fn new() -> IdentityNode {
    IdentityNode { value: 0.0  }
  }
}

impl AudioNode for IdentityNode { 
  fn set_input_value(&mut self, input: i32, value: f32) {
    match input {
      IdentityNode::INPUT_VALUE => self.value = value,
      _ => ()
    };
  }
  fn compute(&mut self) {  }
  fn get_output_value(&self, ouput: i32) -> f32 { 
    match ouput {
      IdentityNode::OUTPUT_VALUE => self.value,
      _ => 0.0
    }
  }
}