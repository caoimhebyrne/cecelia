use std::fmt::Display;

use colored::Colorize;

use crate::{lexer::token::TokenType, position::Position};

pub struct ASTError {
    pub error_type: ASTErrorType,
    pub position: Position,
}

impl ASTError {
    pub fn new(error_type: ASTErrorType, position: Position) -> Self {
        Self {
            error_type,
            position,
        }
    }

    pub fn print_error(&self, input: String) {
        let line = input.lines().nth(self.position.y).unwrap();
        let line_number = self.position.y + 1;
        let column = self.position.x.saturating_sub(1);

        eprintln!(
            "{}",
            format!("Error at line {} column {}: ", line_number, self.position.x)
                .red()
                .bold()
        );

        eprintln!("{}", line.white());
        eprintln!("{}", format!("{}^", " ".repeat(column)).bold());
        eprintln!(
            "{}{}\n",
            " ".repeat(column),
            format!("{}", self.error_type).bold()
        )
    }
}

pub enum ASTErrorType {
    UnableToParse(TokenType),
    UnableToParseValue(TokenType),
    ExpectedToken(TokenType),
    ExpectedTokenButGot(TokenType, TokenType),
    UnexpectedEOF,
}

impl Display for ASTErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTErrorType::UnableToParse(token) => write!(f, "Unable to parse: {:?}", token),
            ASTErrorType::UnableToParseValue(token) => {
                write!(f, "Unable to parse a value from: {:?}", token)
            }
            ASTErrorType::ExpectedToken(token) => write!(f, "Expected token {:?}", token),
            ASTErrorType::ExpectedTokenButGot(expected, actual) => {
                write!(f, "Expected token {:?} but got {:?}", expected, actual)
            }
            ASTErrorType::UnexpectedEOF => write!(f, "Unexpected EOF"),
        }
    }
}
