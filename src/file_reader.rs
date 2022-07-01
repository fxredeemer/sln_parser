use std::fs;

pub struct FileReader {
    path: String,
}

impl FileReader {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn get_contents(&self) -> Result<String, String>{
        match fs::read_to_string(&self.path) {
            Ok(content) => Ok(content),
            Err(_) => Err("Unable to read file contents".to_owned()),
        }
    }
}
