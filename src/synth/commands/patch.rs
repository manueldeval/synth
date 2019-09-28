use std::fs;
use crate::synth::commands::systemcommand::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Patch {
  pub commands: Vec<SystemCommand>
}

impl Patch {
  pub fn new() -> Patch {
    Patch { commands: Vec::new() }
  }

  pub fn from_json(serialized: &String) -> Result<Patch,String> {
    serde_json::from_str(serialized).map_err(|e| format!("{}",e))
  }

  pub fn from_yaml(serialized: &String) -> Result<Patch,String> {
    serde_yaml::from_str(serialized).map_err(|e| format!("{}",e))
  }

  pub fn from_json_file(file_name: &String) -> Result<Patch,String> {
    fs::read_to_string(file_name)
      .map_err(|err| format!("{}",err))
      .and_then(|content| Self::from_json(&content))
  }

  pub fn from_yaml_file(file_name: &String) -> Result<Patch,String> {
    fs::read_to_string(file_name)
      .map_err(|err| format!("{}",err))
      .and_then(|content| Self::from_yaml(&content))
  }

  pub fn from_file(file_name: &String) -> Result<Patch,String> {
    let mut file_name_upper = file_name.to_ascii_uppercase();
    let dot_offset = file_name_upper.find('.').unwrap_or(file_name.len());
    file_name_upper.replace_range(..dot_offset, "");
    match file_name_upper.as_ref() {
      ".YAML" | ".YML" => Patch::from_yaml_file( file_name),
      ".JSON" => Patch::from_json_file( file_name),
      _ => Err(String::from(format!("Unkown format type for file: {}",file_name)))
    }
  }

  pub fn to_json(&self) -> String {
    serde_json::to_string(self).unwrap()
  }

  pub fn to_yaml(&self) -> String {
    serde_yaml::to_string(self).unwrap()
  }

  pub fn add_command(&mut self, command: &SystemCommand) {
    self.commands.push(command.clone());
  }
}

