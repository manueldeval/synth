use crate::patch::patch::*;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;

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

  pub fn load_patch(&self, patch_name: &str) -> Result<Patch,String> {
    let base_path = Path::new(&self.base_path);
    let full_path = base_path.join(format!("{}.yaml",patch_name));
    Patch::from_yaml_file(full_path)
  }

  pub fn save_patch(&self, patch: &Patch, patch_name: &String) -> Result<(),String> {
    let s: String = patch.to_yaml();
    let base_path = Path::new(&self.base_path);
    let full_path = base_path.join(format!("{}.yaml",patch_name));
    let mut file = File::create(full_path).map_err(|e| format!("Unable to save patch {}, cause: {} ",patch_name,e))?;
    file.write_all(s.as_bytes()).map_err(|e| format!("Unable to save patch {}, cause: {} ",patch_name,e))
  }

}

