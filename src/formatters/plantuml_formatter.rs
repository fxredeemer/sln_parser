use std::fmt::Write;
use std::path::Path;

use crate::{constants::LINE_ENDING, formatters::solution_formatter::SolutionFormatter};

pub struct PlantUmlFormatter;

impl PlantUmlFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionFormatter for PlantUmlFormatter {
    fn format(&self, solution: &crate::structures::Solution) -> String {
        let mut output = String::new();

        let header = format!("@startuml{}", LINE_ENDING);
        let footer = "@enduml";

        output += &header;

        for project in solution.projects.iter() {
            let project_name = &project.name;

            if project.dependencies.iter().len() == 0 {
                let _ = write!(&mut output, "[{}]{}", project_name, LINE_ENDING);
            }

            for dependency in project.dependencies.iter() {
                let _ = write!(
                    &mut output,
                    "[{}] ..|> [{}]{}",
                    project_name, dependency, LINE_ENDING
                );
            }
        }

        output += footer;

        output
    }

    fn get_file_ending(&self) -> &Path {
        Path::new("plantuml")
    }
}
