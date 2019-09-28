use serde::{Serialize, Deserialize};
use std::fmt;

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

impl fmt::Display for ConfigVal {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        ConfigVal::FloatVal(v)  => write!(f,"FloatVal: {} )",v),
        ConfigVal::IntVal(v)    => write!(f,"IntVal: {} )",v),
        ConfigVal::StringVal(v) => write!(f,"StringVal: {} )",v),
        ConfigVal::BoolVal(v)   => write!(f,"BoolVal: {} )",v)
      }
    }
}
