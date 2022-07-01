use serde::Serialize;
use std::collections::HashSet;
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Solution {
    pub header: Header,
    pub projects: Vec<Project>,
    pub global_information: GlobalInformation,
}

#[derive(Debug, Serialize)]
pub struct GlobalInformation {
    pub solution_configurations: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct GlobalInformationDraft {
    pub solution_configurations: Vec<String>,
    pub project_configurations_string: String,
}

#[derive(Debug, Serialize)]
pub struct Header {
    pub visual_studio_version: String,
    pub minimum_visual_studio_version: String,
}

#[derive(Debug, Serialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub project_type: Uuid,
    pub dependencies: Vec<String>,
    pub configurations: HashSet<String>,
}

#[derive(Debug)]
pub struct ProjectDraft {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub project_type: Uuid,
    pub dependencies_string: String,
}
