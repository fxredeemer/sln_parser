use crate::{constants::LINE_ENDING, solution_formatter::SolutionFormatter, structures::Solution};
use std::fmt::Write;
use std::path::Path;

pub struct DotFormatter;

impl DotFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl SolutionFormatter for DotFormatter {
    fn format(&self, solution: &Solution) -> String {
        let mut output = String::new();

        let header = format!("digraph dependencies {{ {}", LINE_ENDING);
        let ending = "}";

        output.push_str(&header);

        for project in solution.projects.iter() {
            let project_name = &project.name;

            if project.dependencies.iter().len() == 0 {
                let _ = write!(&mut output, "    \"{}\"{}", project_name, LINE_ENDING);
            }

            for dependency in project.dependencies.iter() {
                let _ = write!(
                    &mut output,
                    "    \"{}\" -> \"{}\"{}",
                    project_name, dependency, LINE_ENDING
                );
            }
        }

        output += ending;

        output
    }

    fn get_file_ending(&self) -> &Path {
        Path::new("dot")
    }
}
