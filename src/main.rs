pub mod ast;
pub mod lexer;
pub mod position;
pub mod stream;

use ast::*;
use clap::{Parser, Subcommand};
use colored::Colorize;
use lexer::*;
use std::{fs, process::exit};
use stream::*;

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
        },
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

fn parse(input: String) {
    let mut lexer = Lexer::new(input.clone());
    let tokens = lexer.parse();
    match tokens {
        Ok(tokens) => {
            let mut ast = AST::new(tokens);
            match ast.parse() {
                Ok(value) => println!("{:#?}", value),
                Err(error) => error.print_error(input),
            }
        },
        Err(error) => error.print_error(input),
    }
}

fn compile(_: String) {
    todo!("Not implemented!")
}
