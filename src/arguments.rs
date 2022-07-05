use std::path::PathBuf;

use clap::{clap_derive::ArgEnum, Parser};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
pub enum Format {
    Dot,
    Json,
    Plantuml,
    //Mermaid
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    #[clap(short = 'i', long = "in-file")]
    pub in_file: PathBuf,
    #[clap(short = 'o', long = "out-folder")]
    pub out_folder: Option<PathBuf>,
    #[clap(short = 'f', long = "out-format", arg_enum)]
    pub out_format: Vec<Format>,
}
