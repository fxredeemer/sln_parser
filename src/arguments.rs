use std::path::PathBuf;

use clap::{Parser, clap_derive::ArgEnum}; 


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum Format {
    Dot,
    //Plantuml,
    //Mermaid
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
 pub struct Arguments{
    #[clap(short = 'i', long = "in-file")]
    in_file: PathBuf,
    #[clap(short = 'o', long = "out-folder")]
    out_folder: Option<PathBuf>,
    #[clap(short = 'f', long = "out-format", arg_enum, default_value_t = Format::Dot)]
    out_format: Format,
 }