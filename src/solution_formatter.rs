use std::path::Path;

use crate::{
    arguments::Format, dot_formatter::DotFormatter, json_formatter::JsonFormatter,
    mermaid_formatter::MermaidFormatter, plantuml_formatter::PlantUmlFormatter,
    structures::Solution,
};

pub trait SolutionFormatter {
    fn format(&self, solution: &Solution) -> String;
    fn get_file_ending(&self) -> &Path;
}

pub struct SolutionFormatterFactory;

impl SolutionFormatterFactory {
    pub fn new() -> Self {
        Self
    }

    pub fn get_formatter(&self, format: &Format) -> Box<dyn SolutionFormatter> {
        match format {
            Format::Dot => Box::new(DotFormatter::new()),
            Format::Json => Box::new(JsonFormatter::new()),
            Format::Plantuml => Box::new(PlantUmlFormatter::new()),
            Format::Mermaid => Box::new(MermaidFormatter::new()),
        }
    }
}
