pub mod ast;
pub mod error;
pub mod lexer;
pub mod position;
pub mod resolver;
pub mod stream;
pub mod r#type;

pub use error::*;

use ast::AST;
use clap::{Parser, Subcommand};
use colored::Colorize;
use lexer::*;
use resolver::{StatementVisitor, TypeResolver};
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
    Check,
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
        Some(Command::Check) => check(input),
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
        Ok(value) => {
            let mut ast = AST::new(value);
            let statements = ast.parse();
            match statements {
                Ok(value) => println!("{:#?}", value),
                Err(error) => error.print_error(input),
            }
        },
        Err(error) => error.print_error(input),
    }
}

fn check(input: String) {
    let mut lexer = Lexer::new(input.clone());
    let tokens = lexer.parse();
    match tokens {
        Ok(value) => {
            let mut ast = AST::new(value);
            let statements = ast.parse();
            match statements {
                Ok(value) => {
                    let mut resolver = TypeResolver::default();
                    let resolved = resolver.visit_statements(value);
                    match resolved {
                        Ok(_values) => println!(
                            "{}: resolved types and checked for errors",
                            "success".to_string().green(),
                        ),
                        Err(error) => error.print_error(input),
                    }
                },
                Err(error) => error.print_error(input),
            }
        },
        Err(error) => error.print_error(input),
    }
}

fn compile(_: String) {
    todo!("Not implemented!")
}
