use crate::structures::{GeneralInformation, Project, ProjectDraft, Solution};
use regex::Regex;
use std::{iter, rc::Rc, vec};
use uuid::Uuid;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_solution_file(&self, content: String) -> Result<Solution, String> {
        let (general_information, remainder) = parse_general_information(&content)?;
        let (projects, remainder) = parse_projects(remainder);

        println!("===========================");
        //println!("{remainder}");

        Ok(Solution {
            general_information,
            projects,
        })
    }
}

fn parse_projects(mut remainder: &str) -> (Vec<Project>, &str) {
    let mut projects = vec![];

    let project_start = format!("EndProject{}", LINE_ENDING);

    while let Some(end_project) = remainder.find(&project_start) {
        let (project_str, remainder_str) = remainder.split_at(end_project + project_start.len());
        remainder = remainder_str;
        if let Some(project) = parse_project(project_str) {
            projects.push(project);
        }
    }

    let projects = map_projects(&projects);

    (projects, remainder)
}

fn parse_project(data: &str) -> Option<ProjectDraft> {
    let regex = Regex::new(r#"^Project\("\{(.+?)\}"\) = "(.+?)", "(.+?)", "\{(.+?)\}""#).unwrap();

    let eol = data.find(LINE_ENDING)?;
    let (project_details, dependencies) = data.split_at(eol);

    let captures = regex.captures(project_details)?;

    let project_type = Uuid::parse_str(&captures[1]).ok()?;
    let name = captures[2].to_owned();
    let path = captures[3].to_owned();
    let id = Uuid::parse_str(&captures[4]).ok()?;

    let dependencies_string = dependencies.to_owned();

    Some(ProjectDraft {
        id,
        name,
        path,
        project_type,
        dependencies_string,
    })
}

fn map_projects(projects: &Vec<ProjectDraft>) -> Vec<Project> {
    let mut out_projects = vec![];

    for draft in projects {
        let mut dependencies = vec![];

        let depencencies_string = &draft.dependencies_string;

        const START_TAG: &str = "ProjectSection";
        const END_TAG: &str = "EndProjectSection";

        let regex = Regex::new(r#"\{(.+?)\} = \{(.+?)\}"#).unwrap();

        if let Some(start) = depencencies_string.find(START_TAG) {
            if let Some(end) = depencencies_string.find(END_TAG) {
                let contents = &depencencies_string[start..end];

                let split = contents.split(LINE_ENDING);

                for entry in split {
                    if let Some(capture) = regex.captures(entry) {
                        if let Ok(id) = Uuid::parse_str(&capture[1]) {
                            if let Some(project) = projects.iter().filter(|d| d.id == id).next() {
                                dependencies.push(project.name.to_owned());
                            }
                        }
                    }
                }

                println!("{contents}");
            }
        }

        let project = Project {
            id: draft.id,
            name: draft.name.clone(),
            path: draft.path.clone(),
            project_type: draft.project_type,
            dependencies,
        };
        out_projects.push(project);
    }
    out_projects
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
