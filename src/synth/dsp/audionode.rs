use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize,Clone)]
pub enum ConfigType {
  FloatType,
  IntType,
  StringType,
  BoolType
}

#[derive(Serialize, Deserialize,Clone)]
pub struct ConfigSpec {
  pub key: String,
  pub typ: ConfigType
}

impl ConfigSpec {
  pub fn new(key: String,typ: ConfigType) -> ConfigSpec {
    ConfigSpec{ key: key, typ: typ }
  }
}

#[derive(Serialize, Deserialize,Clone)]
pub enum ConfigVal {
  FloatVal(f32),
  IntVal(i32),
  StringVal(String),
  BoolVal(bool)
}

impl ConfigVal {
  pub fn as_f32(&self) -> Result<f32,String> {
    if let ConfigVal::FloatVal(f) = self { Ok(*f) } else { Err(String::from("Convertion error.")) }
  }
  pub fn as_i32(&self) -> Result<i32,String> {
    if let ConfigVal::IntVal(f) = self { Ok(*f) } else { Err(String::from("Convertion error.")) }
  }
  pub fn as_bool(&self) -> Result<bool,String> {
    if let ConfigVal::BoolVal(f)  = self { Ok(*f) } else { Err(String::from("Convertion error.")) }
  }
  pub fn as_string(&self) -> Result<String,String> {
    if let ConfigVal::StringVal(f)  = self { Ok((*f).clone()) } else { Err(String::from("Convertion error.")) }
  }
}

pub trait AudioNode {
  
  fn set_config(&mut self, key: &String, val: &ConfigVal) -> Result<(),String> { Ok(()) }
  
  fn get_config_spec() -> Vec<ConfigSpec> where Self: Sized { Vec::new() }

  fn check_key_value_type(key: &String, val: &ConfigVal) -> Result<(),String> where Self: Sized {
    Self::get_config_spec()
      .iter()
      .find(|spec| (*spec).key == *key)
      .and_then(|spec| {
        match (&spec.typ,val){
          (ConfigType::FloatType,ConfigVal::FloatVal(v)) => Some(()),
          (ConfigType::StringType,ConfigVal::StringVal(v)) => Some(()),
          (ConfigType::BoolType,ConfigVal::BoolVal(v)) => Some(()),
          (ConfigType::IntType,ConfigVal::IntVal(v)) => Some(()),
          _ => None
        }
      })
      .ok_or(String::from(""))
  } 

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
