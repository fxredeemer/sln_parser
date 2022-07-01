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
            Ok(_) => todo!(),
            Err(_) => todo!(),
        }
    }
}
