
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct SynthConfig {
  pub osc_port: u16,
  pub osc_ip: String,
  pub web_port: u16,
  pub web_ip: String,
  pub patches_path: String
}

pub fn load_config(path: &str) -> Result<SynthConfig,String> {
  fs::read_to_string(path)
    .map_err(|err| format!("{}",err))
    .and_then(|content| serde_yaml::from_str(&content).map_err(|e| format!("{}",e)))
}

