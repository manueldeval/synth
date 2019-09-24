use crate::synth::dsp::audionode::AudioNode;

pub struct OutputConstantNode {
  value: f32
}

impl OutputConstantNode {
  pub const OUTPUT_CONSTANT: i32 = 0;
  
  pub fn new(value: f32) -> OutputConstantNode {
    OutputConstantNode { value: value }
  }
}

impl AudioNode for OutputConstantNode {
  
  fn set_input_value(&mut self, _input: i32, _value: f32) { }
  fn compute(&mut self) { }
  fn get_output_value(&self, _ouput: i32) -> f32 { self.value }
}
