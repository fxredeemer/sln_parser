use dot_formatter::DotFormatter;
use file_reader::FileHandler;
use parser::Parser;

mod constants;
mod dot_formatter;
mod file_reader;
mod parser;
mod structures;

fn main() -> Result<(), String> {
    let file_handler = FileHandler::new();
    let parser = Parser::new();

    let content = file_handler.get_contents("samples/Files.sln")?;
    let solution = parser.parse_solution_file(content)?;

    let serialized = serde_json::to_string_pretty(&solution).unwrap();
    file_handler.save_to_file("samples/files.json", serialized)?;

    let dot_exporter = DotFormatter::new();
    let dot_output = dot_exporter.export(&solution);
    file_handler.save_to_file("samples/graph.dot", dot_output)?;

    Ok(())
}
