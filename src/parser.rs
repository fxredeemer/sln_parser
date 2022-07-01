use crate::structures::{GlobalInformation, Header, Project, ProjectDraft, Solution};
use regex::Regex;
use std::vec;
use uuid::Uuid;

#[cfg(windows)]
const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &str = "\n";

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_solution_file(&self, content: String) -> Result<Solution, String> {
        let (header, remainder) = parse_general_information(&content)?;
        let (projects, remainder) = parse_projects(remainder);
        let global_information = parse_global_information(remainder)?;

        Ok(Solution {
            header,
            projects,
            global_information,
        })
    }
}

fn parse_projects(mut remainder: &str) -> (Vec<Project>, &str) {
    let mut projects = vec![];

    let start_tag = format!("EndProject{}", LINE_ENDING);

    while let Some(end_project) = remainder.find(&start_tag) {
        let (project_str, remainder_str) = remainder.split_at(end_project + start_tag.len());
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
                            if let Some(project) = projects.iter().find(|d| d.id == id) {
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
            configurations: vec![],
        };
        out_projects.push(project);
    }
    out_projects
}

fn parse_global_information(data: &str) -> Result<GlobalInformation, String> {
    let global_part = get_global_part(data)?;
    let solution_configuration_data = get_solution_configuration_part(global_part)?;

    let mut solution_configurations = vec![];
    let lines = solution_configuration_data.split(LINE_ENDING);
    
    let regex = Regex::new(r#"(\w+\|\w+) = \w+\|\w+"#).unwrap();
    for line in lines {
        if let Some(captures) = regex.captures_iter(line).next(){
            solution_configurations.push(captures[1].to_owned());
        }
    }

    Ok(GlobalInformation {
        solution_configurations,
    })
}

fn get_solution_configuration_part(global_part: &str) -> Result<&str, String> {
    let section_start_tag = "GlobalSection(SolutionConfigurationPlatforms)";
    let section_end_tag = "EndGlobalSection";
    let section_start = global_part
        .find(&section_start_tag)
        .ok_or("Unable to find Start of Global Section SolutionConfigurationPlatforms")?;
    let data = &global_part[section_start..];
    let section_end = data
        .find(&section_end_tag)
        .ok_or("Unable to find End of Global Section SolutionConfigurationPlatforms")?;
    Ok(&global_part[..section_end])
}

fn get_global_part(data: &str) -> Result<&str, String> {
    let start_tag = format!("Global{}", LINE_ENDING);
    let end_tag = format!("EndGlobal{}", LINE_ENDING);
    let start = data
        .find(&start_tag)
        .ok_or("Unable to find Start of Global")?;
    let end = data.find(&end_tag).ok_or("Unable to find End of Global")?;
    Ok(&data[start..end])
}

fn parse_general_information(content: &str) -> Result<(Header, &str), String> {
    let start_project = content
        .find("Project(")
        .ok_or("Could not read first Project")?;
    let (general_information, remainder) = content.split_at(start_project);
    println!("{general_information}");

    Ok((
        Header {
            visual_studio_version: general_information[..1].to_owned(),
            minimum_visual_studio_version: general_information[..1].to_owned(),
        },
        remainder,
    ))
}
