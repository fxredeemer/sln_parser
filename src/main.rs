use file_reader::FileReader;
use parser::Parser;

mod file_reader;
mod parser;
mod structures;

fn main() -> Result<(), String> {
    let file_reader = FileReader::new("samples/Files.sln".to_owned());
    let parser = Parser::new();

    let content = file_reader.get_contents()?;
    let solution = parser.parse_solution_file(content)?;

    let serialized = serde_json::to_string_pretty(&solution).unwrap();
    println!("{serialized}");

    Ok(())
}
