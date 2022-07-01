use std::fmt::Debug;
use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Solution {
    pub projects: Vec<Project>,
    pub general_information: GeneralInformation,
}

#[derive(Debug, Serialize)]
pub struct GeneralInformation {
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
}

#[derive(Debug)]
pub struct ProjectDraft {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub project_type: Uuid,
    pub dependencies_string: String,
}
