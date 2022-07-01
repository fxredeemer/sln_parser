use file_reader::FileReader;
use parser::Parser;

mod structures;
mod parser;
mod file_reader;

fn main() -> Result<(), String> {

    let file_reader = FileReader::new("".to_owned());
    let parser = Parser::new();

    let content = file_reader.get_contents()?;
    let _solution = parser.parse_solution_file(content)?;

    Ok(())
}
