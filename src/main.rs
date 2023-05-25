use anyhow::bail;
use clap::Parser;
use colored::Colorize;
use interpreter::{FuncDef, Interpreter};
use lower::Lower;
use std::{error::Error, fs, path::PathBuf, process::ExitCode};

use js::AstParser;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub js);

mod ast;
mod interpreter;
mod lower;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    file: PathBuf,
}

fn main() -> ExitCode {
    let args = Args::parse();

    match run(args) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{}", err);
            ExitCode::FAILURE
        }
    }
}

fn run(args: Args) -> Result<(), anyhow::Error> {
    let content = fs::read_to_string(args.file)?;

    let parser = AstParser::new();

    let result = match parser.parse(&content) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("{}", err);
            bail!("Compilation failed".bold().red())
        }
    };

    let mut interpreter = Interpreter::new();

    for func in result.root.functions {
        let lower = Lower::new();

        let name_key = interpreter.register_literal(func.name.clone());

        let func_def = lower.lower(name_key, func);
        interpreter.add_function(func_def);
    }

    interpreter.execute()?;

    Ok(())
}
