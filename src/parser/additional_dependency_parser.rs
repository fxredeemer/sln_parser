use crate::{file_reader::FileHandler, structures::Project};
use regex::Regex;
use std::path::Path;

pub struct AdditionalDependencyParser;

impl AdditionalDependencyParser {
    pub fn new() -> Self {
        Self
    }

    pub fn fun_name(&self, projects: &mut [Project], sln_path: &Path) {
        for project in projects.iter_mut() {
            let mut path = sln_path.parent().unwrap().to_owned();
            path.push(&project.path);

            if let Some(extension) = path.extension() {
                if extension == "csproj" {
                    let file_handler = FileHandler::new();
                    let contents = file_handler.get_contents(&path).unwrap();

                    let reference_mutex =
                        Regex::new(r#"<ProjectReference.+Include="(.+?)".+>"#).unwrap();

                    for capture in reference_mutex.captures_iter(&contents) {
                        let path = Path::new(&capture[1]);
                        let dependency = path.file_stem().unwrap().to_str().unwrap();

                        project.dependencies.push(dependency.to_owned());
                    }
                }
            }
        }
    }
}
