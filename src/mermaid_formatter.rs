use std::fmt::Write;
use std::path::Path;

use crate::{constants::LINE_ENDING, solution_formatter::SolutionFormatter};

pub struct MermaidFormatter;

impl MermaidFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionFormatter for MermaidFormatter {
    fn format(&self, solution: &crate::structures::Solution) -> String {
        let mut output = String::new();

        let header = format!("graph TD;{}", LINE_ENDING);

        output.push_str(&header);

        for project in solution.projects.iter() {
            let project_name = &project.name;

            if project.dependencies.iter().len() == 0 {
                let _ = write!(&mut output, "    {};{}", project_name, LINE_ENDING);
            }

            for dependency in project.dependencies.iter() {
                let _ = write!(
                    &mut output,
                    "    {} --> {};{}",
                    project_name, dependency, LINE_ENDING
                );
            }
        }

        output
    }

    fn get_file_ending(&self) -> &Path {
        Path::new("mermaid")
    }
}
