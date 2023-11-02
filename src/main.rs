pub mod lexer;
use colored::Colorize;
pub use lexer::*;

pub mod stream;
pub use stream::*;

use clap::{Parser, Subcommand};
use std::{fs, process::exit};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,

    #[arg(global = true)]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    Lex,
    Parse,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let input = match args.file {
        Some(value) => fs::read_to_string(value)?,
        None => {
            eprintln!("{}: No input file specified!", "error".red());
            exit(-1);
        }
    };

    match args.command {
        Some(Command::Lex) => lex(input),
        Some(Command::Parse) => parse(input),
        None => compile(input),
    }

    Ok(())
}

fn lex(input: String) {
    let mut lexer = Lexer::new(input.clone());
    let tokens = lexer.parse();
    match tokens {
        Ok(value) => println!("{:#?}", value),
        Err(error) => error.print_error(input),
    }
}

fn parse(_: String) {
    todo!("Not implemented!")
}

fn compile(_: String) {
    todo!("Not implemented!")
}
