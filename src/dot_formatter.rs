use crate::{constants::LINE_ENDING, structures::Solution};

pub struct DotFormatter;

impl DotFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn export(&self, solution: &Solution) -> String {
        let mut output = String::new();

        let header = format!("digraph dependencies {{ {}", LINE_ENDING);
        let ending = "}";

        output += &header;

        for project in solution.projects.iter() {
            let project_name = &project.name;

            if project.dependencies.iter().len() == 0 {
                output += &format!("    \"{}\"{}", project_name, LINE_ENDING);
            }

            for dependency in project.dependencies.iter() {
                output += &(format!(
                    "    \"{}\" -> \"{}\"{}",
                    project_name, dependency, LINE_ENDING
                ));
            }
        }

        output += ending;

        output
    }
}
