use std::fs::File;
use std::io::Read;
use std::path::Path;
use clap::{Arg, Command, ValueEnum};
use clap::builder::{EnumValueParser, PossibleValue};
use crate::db::DBHeader;

mod utils;
mod test;
mod db;

#[derive(Clone, Debug)]
enum ArgOption {
    DBInfo,
    Tables,
}

impl ValueEnum for ArgOption {
    fn value_variants<'a>() -> &'a [Self] {
        return &[ArgOption::DBInfo, ArgOption::Tables];
    }

    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        let mut actual_input = String::from(input);
        if ignore_case {
            actual_input = actual_input.to_lowercase();
        }
        match &actual_input[..] {
            ".dbinfo" => Ok(ArgOption::DBInfo),
            ".tables" => Ok(ArgOption::Tables),
            _ => Err(String::from("No such type of arg.")),
        }
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::DBInfo => PossibleValue::new(".dbinfo").help("Print .db file info."),
            Self::Tables => PossibleValue::new(".tables").help("Print .db file table names.")
        })
    }
}

fn main() {
    // TODO: implement the required = false functionality
    let m = Command::new("Rsqlite")
        .author("Criswe11, fanana.bass@gmail.com")
        .version("0.1.0")
        .about("Rust powered SQLite-like DBMS.")
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .required(true).help("Path of .db file which you hope to operate.")
        ).arg(
        Arg::new("option")
            .value_name("OPTION")
            .required(true)
            .value_parser(EnumValueParser::<ArgOption>::new())
    ).get_matches();
    let mut f = File::open(m.get_one::<String>("path").unwrap()).expect("File does not exist.");
    match m.get_one::<ArgOption>("option").unwrap() {
        ArgOption::DBInfo => print_db_info(f),
        ArgOption::Tables => print_tables(f),
    }
}

fn print_db_info(mut f: File) {
    let mut header = [0u8; 100];
    f.read_exact(&mut header).expect("Read .db file error.");
    let header:DBHeader = DBHeader::from(&header);
    println!("{}", header);
}

fn print_tables(mut f: File) {}
