use crate::synth::dsp::audionode::AudioNode;

/*
=========================================
Null
=========================================
*/

pub struct NullTerminalNode {

}

impl NullTerminalNode {
  pub fn new() -> NullTerminalNode {
    NullTerminalNode {  }
  }
}

impl AudioNode for NullTerminalNode { 
  fn set_input_value(&mut self, _input: i32, _value: f32) { }
  fn compute(&mut self) {  }
  fn get_output_value(&self, _ouput: i32) -> f32 { 0.0 }
}