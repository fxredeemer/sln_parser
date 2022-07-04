use std::path::PathBuf;

use arguments::Arguments;
use clap::Parser;
use file_reader::FileHandler;
use sln_parser::SlnParser;

use crate::solution_formatter::SolutionFormatterFactory;

mod arguments;
mod constants;
mod dot_formatter;
mod file_reader;
mod json_formatter;
mod sln_parser;
mod solution_formatter;
mod structures;

fn main() -> Result<(), String> {
    let arguments = Arguments::parse();
    print!("{:?}", arguments);

    let file_handler = FileHandler::new();
    let parser = SlnParser::new();

    let in_file = &arguments.in_file;

    let content = file_handler.get_contents(in_file)?;
    let solution = parser.parse_solution_file(content)?;

    let solution_formatter_factory = SolutionFormatterFactory::new();

    println!("");

    for format in arguments.out_format.iter() {
        let mut out_path = get_out_folder(&arguments)?;
        let formatter = solution_formatter_factory.get_formatter(format);
        let file_ending = formatter.get_file_ending();

        out_path.push(&arguments.in_file.file_stem().unwrap());
        out_path.set_extension(file_ending);

        println!("OUT_FILE  {}", &out_path.to_str().unwrap());

        let contents = formatter.format(&solution);
        file_handler.save_to_file(&out_path, contents)?;
    }

    Ok(())
}

fn get_out_folder(arguments: &Arguments) -> Result<PathBuf, String> {
    let folder = match &arguments.out_folder {
        Some(folder) => folder,
        None => match arguments.in_file.parent() {
            Some(path) => path,
            None => return Err("Invalid file location".to_owned()),
        },
    };

    Ok(folder.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arguments::Format;
    use std::{path::Path, str::FromStr};

    #[test]
    fn get_out_folder_no_out_folder_parent_of_in_selected() {
        let arguments = Arguments {
            in_file: PathBuf::from_str("C:\\asdf.sln").unwrap(),
            out_folder: None,
            out_format: vec![Format::Dot],
        };

        let out_folder = get_out_folder(&arguments).unwrap();

        assert_eq!(out_folder, Path::new("C:\\"));
    }

    #[test]
    fn get_out_folder_out_folder_present_out_folder_selected() {
        let arguments = Arguments {
            in_file: PathBuf::from_str("C:\\asdf.sln").unwrap(),
            out_folder: Some(PathBuf::from_str("C:\\qwert").unwrap()),
            out_format: vec![Format::Dot],
        };

        let out_folder = get_out_folder(&arguments).unwrap();

        assert_eq!(out_folder, Path::new("C:\\qwert"));
    }
}
