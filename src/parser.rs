use crate::structures::Solution;

pub struct Parser;

impl Parser {
    pub fn new() -> Self{
        Self
    }

    pub fn parse_solution_file(&self, _content: String) -> Result<Solution, String> {
        Err("A".to_owned())
    }
}
