use crate::{file_reader::FileHandler, structures::Project};
use regex::Regex;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub struct AdditionalDependencyParser;

impl AdditionalDependencyParser {
    pub fn new() -> Self {
        Self
    }

    pub fn get_additional_dependencies_from_project_files(
        &self,
        projects: &mut [Project],
        sln_path: &Path,
    ) {
        for project in projects.iter_mut() {
            if let Some(path) = try_get_csproj(project, sln_path) {
                if let Some(dependencies) = get_dependencies_from_csproj(&path) {
                    for dependency in dependencies {
                        project.dependencies.push(dependency)
                    }
                }
            }
        }
    }
}

fn try_get_csproj(project: &mut Project, sln_path: &Path) -> Option<PathBuf> {
    if let Some(parent) = sln_path.parent() {
        let mut path = parent.to_owned();
        path.push(&project.path);

        if let Some(extension) = path.extension() {
            if extension == "csproj" {
                return Some(path);
            }
        }
    }

    None
}

fn get_dependencies_from_csproj(path: &Path) -> Option<Vec<String>> {
    let mut dependencies = vec![];
    let file_handler = FileHandler::new();
    let contents = file_handler.get_contents(path).ok()?;
    let reference_mutex = Regex::new(r#"<ProjectReference.+Include="(.+?)".+>"#).unwrap();

    for capture in reference_mutex.captures_iter(&contents) {
        let path = Path::new(&capture[1]);
        let dependency = path.file_stem().and_then(OsStr::to_str);
        push_if_some(&mut dependencies, dependency);
    }

    Some(dependencies)
}

fn push_if_some(dependencies: &mut Vec<String>, optional_value: Option<&str>) {
    if let Some(value) = optional_value {
        dependencies.push(value.to_owned());
    }
}
