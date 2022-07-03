use crate::structures::{
    GlobalInformation, GlobalInformationDraft, Header, Project, ProjectDraft, Solution,
};
use regex::Regex;
use std::{collections::HashSet, vec};
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
        let header_part = get_header_part(&content)?;
        let header = parse_general_information(header_part)?;

        let projects_part = get_projects_part(&content)?;
        let projects = parse_project_drafts(projects_part);

        let global_part = get_global_part(&content)?;
        let global_information = parse_global_information_draft(global_part)?;

        let mut projects = wire_project_dependencies(&projects);

        set_project_configurations(&global_information, &mut projects);

        let global_information = GlobalInformation {
            solution_configurations: global_information.solution_configurations,
        };

        Ok(Solution {
            header,
            projects,
            global_information,
        })
    }
}

fn get_header_part(content: &String) -> Result<&str, String> {
    let header_end_tag = "Project(";
    let header_end = content
        .find(header_end_tag)
        .ok_or("Unable To determine Header")?;
    Ok(&content[..header_end])
}

fn get_projects_part(content: &str) -> Result<&str, String> {
    let start_tag = "Project(";
    let end_tag = &format!("EndProject{}", LINE_ENDING);
    let projects_start = content
        .find(start_tag)
        .ok_or("Can not find Projects Part")?;
    let mut projects_end = content.rfind(end_tag).ok_or("Can not find Projects Part")?;
    projects_end += end_tag.len();
    let projects_part = &content[projects_start..projects_end];
    Ok(projects_part)
}

fn set_project_configurations(
    global_information: &GlobalInformationDraft,
    projects: &mut [Project],
) {
    let lines = global_information
        .project_configurations_string
        .split(LINE_ENDING);

    let regex = Regex::new(r#"\{(.+)\}\..+ = (\w+\|\w+)"#).unwrap();
    for line in lines {
        if let Some(captures) = regex.captures(line) {
            if let Ok(id) = Uuid::parse_str(&captures[1]) {
                if let Some(project) = projects.iter_mut().find(|d| d.id == id) {
                    project.configurations.insert(captures[2].to_owned());
                }
            }
        }
    }
}

fn parse_project_drafts(mut remainder: &str) -> Vec<ProjectDraft> {
    let mut projects = vec![];

    let start_tag = format!("EndProject{}", LINE_ENDING);

    while let Some(end_project) = remainder.find(&start_tag) {
        let (project_str, remainder_str) = remainder.split_at(end_project + start_tag.len());

        remainder = remainder_str;
        if let Some(project) = parse_project(project_str) {
            projects.push(project);
        }
    }

    projects
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

fn wire_project_dependencies(projects: &Vec<ProjectDraft>) -> Vec<Project> {
    let mut out_projects = vec![];

    for draft in projects {
        let dependencies = parse_dependencies(draft, projects);

        let project = Project {
            id: draft.id,
            name: draft.name.clone(),
            path: draft.path.clone(),
            project_type: draft.project_type,
            dependencies,
            configurations: HashSet::new(),
        };
        out_projects.push(project);
    }
    out_projects
}

fn parse_dependencies(draft: &ProjectDraft, projects: &[ProjectDraft]) -> Vec<String> {
    let mut dependencies = vec![];
    let depencencies_string = &draft.dependencies_string;

    const START_TAG: &str = "ProjectSection(ProjectDependencies";
    let end_tag = &format!("EndProjectSection{}", LINE_ENDING);

    if let Some(start) = depencencies_string.find(START_TAG) {
        if let Some(end) = depencencies_string.find(end_tag) {
            let regex = Regex::new(r#"\{(.+?)\} = \{.+?\}"#).unwrap();
            let contents = &depencencies_string[start..end];

            let lines = contents.split(LINE_ENDING);

            for line in lines {
                if let Some(capture) = regex.captures(line) {
                    if let Ok(id) = Uuid::parse_str(&capture[1]) {
                        if let Some(project) = projects.iter().find(|d| d.id == id) {
                            dependencies.push(project.name.to_owned());
                        }
                    }
                }
            }
        }
    }
    dependencies
}

fn parse_global_information_draft(global_part: &str) -> Result<GlobalInformationDraft, String> {
    let solution_configuration_part = get_solution_configuration_part(global_part)?;
    let solution_configurations = get_solution_configurations(solution_configuration_part);
    let project_configurations_string = get_project_configuration_part(global_part)?;

    Ok(GlobalInformationDraft {
        solution_configurations,
        project_configurations_string,
    })
}

fn get_project_configuration_part(global_part: &str) -> Result<String, String> {
    let section_start_tag = "GlobalSection(ProjectConfigurationPlatforms)";
    let section_end_tag = "EndGlobalSection";

    let section_start = global_part
        .find(&section_start_tag)
        .ok_or("Unable to find Start of Global Section ProjectConfigurationPlatforms")?;
    let data = &global_part[section_start..];
    let section_end = data
        .find(&section_end_tag)
        .ok_or("Unable to find End of Global Section ProjectConfigurationPlatforms")?;

    Ok(global_part[..section_end].to_owned())
}

fn get_solution_configurations(solution_configuration_data: &str) -> Vec<String> {
    let mut solution_configurations = vec![];
    let lines = solution_configuration_data.split(LINE_ENDING);
    let regex = Regex::new(r#"(\w+\|\w+) = \w+\|\w+"#).unwrap();
    for line in lines {
        if let Some(captures) = regex.captures_iter(line).next() {
            solution_configurations.push(captures[1].to_owned());
        }
    }
    solution_configurations
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

fn parse_general_information(general_information: &str) -> Result<Header, String> {
    Ok(Header {
        visual_studio_version: general_information[..1].to_owned(),
        minimum_visual_studio_version: general_information[..1].to_owned(),
    })
}
