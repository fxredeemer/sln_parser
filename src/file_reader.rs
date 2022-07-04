use std::{fs, path::Path};

pub struct FileHandler;

impl FileHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn get_contents(&self, in_path: &Path) -> Result<String, String> {
        match fs::read_to_string(in_path) {
            Ok(content) => Ok(content),
            Err(_) => Err("Unable to read file contents".to_owned()),
        }
    }

    pub fn save_to_file(&self, out_path: &Path, data: String) -> Result<(), String> {
        match fs::write(out_path, data) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to write file".to_owned()),
        }
    }
}
