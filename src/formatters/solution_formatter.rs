use std::path::Path;

use crate::{
    arguments::Format, formatters::dot_formatter::DotFormatter,
    formatters::json_formatter::JsonFormatter, formatters::mermaid_formatter::MermaidFormatter,
    formatters::plantuml_formatter::PlantUmlFormatter, structures::Solution,
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
