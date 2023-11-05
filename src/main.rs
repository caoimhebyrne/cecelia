pub mod ast;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod position;
pub mod resolver;
pub mod stream;
pub mod r#type;
pub mod visitor;

pub use error::*;

use ast::*;
use clap::{Parser, Subcommand};
use colored::Colorize;
use interpreter::Interpreter;
use lexer::*;
use resolver::*;
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

    let input = match &args.file {
        Some(value) => fs::read_to_string(value)?,
        None => {
            eprintln!("{}: No input file specified!", "error".red());
            exit(-1);
        },
    };

    let result = match args.command {
        Some(Command::Lex) => lex(input.clone()),
        Some(Command::Parse) => parse(input.clone()),
        Some(Command::Check) => check(input.clone()),
        None => execute(input.clone()),
    };

    if let Err(error) = result {
        error.print_error(input);
        exit(-1);
    }

    Ok(())
}

fn lex(input: String) -> Result<(), Error> {
    let mut lexer = Lexer::new(input.clone());
    let tokens = lexer.parse()?;

    println!("{:#?}", tokens);
    Ok(())
}

fn parse(input: String) -> Result<(), Error> {
    let mut lexer = Lexer::new(input.clone());
    let tokens = lexer.parse()?;

    let mut ast = AST::new(tokens);
    let statements = ast.parse()?;

    println!("{:#?}", statements);
    Ok(())
}

fn check(input: String) -> Result<(), Error> {
    let mut lexer = Lexer::new(input.clone());
    let tokens = lexer.parse()?;

    let mut ast = AST::new(tokens);
    let mut statements = ast.parse()?;

    let mut resolver = TypeResolver::default();
    statements = resolver.visit_statements(statements)?;

    println!("{}: No errors found!", "success".green());
    println!("{:#?}", statements);

    Ok(())
}

fn execute(input: String) -> Result<(), Error> {
    let mut lexer = Lexer::new(input.clone());
    let tokens = lexer.parse()?;

    let mut ast = AST::new(tokens);
    let mut statements = ast.parse()?;

    let mut resolver = TypeResolver::default();
    statements = resolver.visit_statements(statements)?;

    let mut interpreter = Interpreter::default();
    interpreter.visit_statements(statements)?;

    interpreter.print_variables();
    Ok(())
}
