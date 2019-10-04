use crate::synth::commands::config::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Clone)]
pub struct ConnectorSpec {
  name: String,
  description: String
}

impl ConnectorSpec {
  pub fn new(name: String, description: String) -> ConnectorSpec {
    ConnectorSpec { name:name, description:description }
  }
}

#[derive(Serialize, Deserialize,Clone)]
pub struct IOSpec {
  pub inputs: Vec<ConnectorSpec>,
  pub outputs: Vec<ConnectorSpec>
}

impl IOSpec {
  pub fn new(inputs: Vec<ConnectorSpec>,outputs: Vec<ConnectorSpec>) -> IOSpec {
    IOSpec{inputs:inputs,outputs:outputs}
  }
  pub fn empty() -> IOSpec {
    IOSpec{inputs: Vec::new(),outputs: Vec::new()}
  }
} 

pub trait AudioNode {
  
  fn set_config(&mut self, _key: &String, _val: &ConfigVal) -> Result<(),String> { Ok(()) }
  
  // fn get_config_spec() -> Vec<ConfigSpec> where Self: Sized { Vec::new() }

  // fn check_key_value_type(key: &String, val: &ConfigVal) -> Result<(),String> where Self: Sized {
  //   Self::get_config_spec()
  //     .iter()
  //     .find(|spec| (*spec).key == *key)
  //     .and_then(|spec| {
  //       match (&spec.typ,val){
  //         (ConfigType::FloatType,ConfigVal::FloatVal(_v)) => Some(()),
  //         (ConfigType::StringType,ConfigVal::StringVal(_v)) => Some(()),
  //         (ConfigType::BoolType,ConfigVal::BoolVal(_v)) => Some(()),
  //         (ConfigType::IntType,ConfigVal::IntVal(_v)) => Some(()),
  //         _ => None
  //       }
  //     })
  //     .ok_or(String::from(""))
  // } 

  fn set_sample_rate(&mut self,_frequency: i32) {}

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
