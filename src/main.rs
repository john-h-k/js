use clap::Parser;
use std::{fs, path::PathBuf};

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub js);

mod ast;
mod codegen;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let parser = js::StatementParser::new();

    let text = fs::read_to_string(args.file).unwrap();
    dbg!(parser.parse(&text).unwrap());
}
