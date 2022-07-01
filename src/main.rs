use file_reader::FileHandler;
use parser::Parser;

mod file_reader;
mod parser;
mod structures;

fn main() -> Result<(), String> {
    let file_handler = FileHandler::new("samples/Files.sln", "samples/files.json");
    let parser = Parser::new();

    let content = file_handler.get_contents()?;
    let solution = parser.parse_solution_file(content)?;

    let serialized = serde_json::to_string_pretty(&solution).unwrap();
    file_handler.save_to_file(serialized)?;

    Ok(())
}
