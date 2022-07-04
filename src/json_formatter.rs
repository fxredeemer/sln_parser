use std::path::Path;

use crate::solution_formatter::SolutionFormatter;
use serde_json::*;

pub struct JsonFormatter;

impl JsonFormatter {
    pub fn new() -> Self {
        Self
    }
}
impl SolutionFormatter for JsonFormatter {
    fn format(&self, solution: &crate::structures::Solution) -> String {
        to_string_pretty(&solution).unwrap()
    }

    fn get_file_ending(&self) -> &Path {
        Path::new("json")
    }
}
