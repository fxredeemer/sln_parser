use crate::structures::{GeneralInformation, Project, ProjectDraft, Solution};
use regex::Regex;
use std::vec;
use uuid::Uuid;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_solution_file(&self, content: String) -> Result<Solution, String> {
        let (general_information, remainder) = parse_general_information(&content)?;
        let (projects, remainder) = parse_projects(remainder);

        println!("===========================");
        println!("{remainder}");

        Ok(Solution {
            general_information,
            projects,
        })
    }
}

fn parse_projects(mut remainder: &str) -> (Vec<Project>, &str) {
    let mut projects = vec![];
    while let Some(end_project) = remainder.find("EndProject\r\n") {
        let (project_str, remainder_str) = remainder.split_at(end_project + "EndProject\r\n".len());
        remainder = remainder_str;
        if let Some(project) = parse_project(project_str) {
            projects.push(project);
        }
    }

    let projects = projects.iter().map(|p| map_project(p, &projects)).collect();

    (projects, remainder)
}

fn parse_project(data: &str) -> Option<ProjectDraft> {
    println!("---------------------------");
    let regex = Regex::new(r#"^Project\("\{(.+?)\}"\) = "(.+?)", "(.+?)", "\{(.+?)\}""#).unwrap();

    let eol = data.find("\r\n")?;
    let (project_details, dependencies) = data.split_at(eol);

    let captures = regex.captures(project_details)?;
    let project_type = Uuid::parse_str(&captures[1]).ok()?;
    println!("project_type: {project_type}");

    let name = captures[2].to_owned();
    println!("name: {name}");

    let path = captures[3].to_owned();
    println!("path: {path}");

    let id = Uuid::parse_str(&captures[4]).ok()?;
    println!("id: {id}");

    println!("Dependencies : {dependencies}");
    println!("---------------------------");

    let dependencies = dependencies.to_owned();

    Some(ProjectDraft {
        id,
        name,
        path,
        project_type,
        dependencies,
    })
}

fn map_project(draft: &ProjectDraft, projects: &Vec<ProjectDraft>) -> Project {
    let dependencies = vec![];

    Project {
        id: draft.id,
        name: draft.name.clone(),
        path: draft.path.clone(),
        project_type: draft.project_type,
        dependencies,
    }
}

fn parse_general_information(content: &str) -> Result<(GeneralInformation, &str), String> {
    let start_project = content
        .find("Project(")
        .ok_or("Could not read first Project")?;
    let (general_information, remainder) = content.split_at(start_project);
    println!("{general_information}");

    Ok((
        GeneralInformation {
            visual_studio_version: general_information[..1].to_owned(),
            minimum_visual_studio_version: general_information[..1].to_owned(),
        },
        remainder,
    ))
}
