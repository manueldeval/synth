use crate::patch::patch::*;
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct PatchManager {
  base_path: String,
}

impl PatchManager {
  pub fn new(base_path: &str) -> PatchManager {
    PatchManager { base_path: String:: from(base_path) }
  }

  pub fn patches(&self) -> Result<Vec<String>,String> {
    let path =  Path::new(&self.base_path);
    if ! path.exists() {
      Err(String::from("The patch path does not exists."))
    } else if ! path.is_dir() {
      Err(String::from("The patch path is not a directory."))
    } else {
      fs::read_dir(path)
        .map_err(|_| String::from(format!("The patch path cannot be read.")))
        .map(|files| {
          files.flat_map(|e| e)
          .flat_map(|e| e.file_name().into_string().into_iter())
          .filter(PatchManager::is_patch_extension)
          .map(PatchManager::remove_extension)
          .collect()
        })                 
    }
  }

  pub fn remove_extension(file_name: String) -> String {
    let mut file_name_upper = file_name.to_ascii_uppercase();
    let dot_offset = file_name_upper.find('.').unwrap_or(file_name.len());
    file_name_upper.replace_range(dot_offset .. file_name.len(), "");
    file_name_upper.clone()
  }

  pub fn is_patch_extension(file_name: &String) -> bool {
    let mut file_name_upper = file_name.to_ascii_uppercase();
    let dot_offset = file_name_upper.find('.').unwrap_or(file_name.len());
    file_name_upper.replace_range(..dot_offset, "");
    file_name == ".yaml"
  }

}

