


pub trait AudioNode {
  
  fn configure(&mut self,_frequency: i32) {}

  fn set_input_value(&mut self, input: i32, value: f32);

  fn compute(&mut self);

  fn get_output_value(&self,ouput: i32) -> f32;
}

/*
=========================================
CONSTANT
=========================================
*/

pub struct ConstantNode {
  value: f32
}

impl ConstantNode {
  pub const OUTPUT_CONSTANT: i32 = 0;
  
  pub fn new(value: f32) -> ConstantNode {
    ConstantNode { value: value }
  }
}

impl AudioNode for ConstantNode {
  
  fn set_input_value(&mut self, _input: i32, _value: f32) { }
  fn compute(&mut self) { }
  fn get_output_value(&self, _ouput: i32) -> f32 { self.value }
}

/*
=========================================
Null
=========================================
*/

pub struct NullNode {

}

impl NullNode {
  pub fn new() -> NullNode {
    NullNode {  }
  }
}

impl AudioNode for NullNode { 
  fn set_input_value(&mut self, _input: i32, _value: f32) { }
  fn compute(&mut self) {  }
  fn get_output_value(&self, _ouput: i32) -> f32 { 0.0 }
}
