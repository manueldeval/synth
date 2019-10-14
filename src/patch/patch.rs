use std::fs;
use crate::synth::commands::systemcommand::*;
use crate::patch::meta::*;

use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Patch {
  pub commands: Vec<SystemCommand>,
  pub metas: Vec<WidgetMeta>
}

impl Patch {
  pub fn new() -> Patch {
    Patch { commands: Vec::new(), metas: Vec::new() }
  }

  pub fn from_json(serialized: &String) -> Result<Patch,String> {
    serde_json::from_str(serialized).map_err(|e| format!("{}",e))
  }

  pub fn from_yaml(serialized: &String) -> Result<Patch,String> {
    serde_yaml::from_str(serialized).map_err(|e| format!("{}",e))
  }

  pub fn from_json_file<P: AsRef<Path>>(file_name: P) -> Result<Patch,String> {
    let file = file_name.as_ref().as_os_str().to_str().unwrap_or("Unkown");

    fs::read_to_string(file)
      .map_err(|err| format!("Unable to load patch {} : {}",&file, err))
      .and_then(|content| Self::from_json(&content))
  }

  pub fn from_yaml_file<P: AsRef<Path>>(file_name: P) -> Result<Patch,String> {
    
    let file = file_name.as_ref().as_os_str().to_str().unwrap_or("Unkown");

    fs::read_to_string(file)
      .map_err(|err| format!("Unable to load patch {} : {}",&file, err))
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

