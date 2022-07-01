use std::fs;

pub struct FileHandler {
    in_path: String,
    out_path: String,
}

impl FileHandler {
    pub fn new(in_path: &str, out_path: &str) -> Self {
        Self {
            in_path: in_path.to_owned(),
            out_path: out_path.to_owned(),
        }
    }

    pub fn get_contents(&self) -> Result<String, String> {
        match fs::read_to_string(&self.in_path) {
            Ok(content) => Ok(content),
            Err(_) => Err("Unable to read file contents".to_owned()),
        }
    }

    pub fn save_to_file(&self, data: String) -> Result<(), String> {
        match fs::write(&self.out_path, data) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to write file".to_owned()),
        }
    }
}
