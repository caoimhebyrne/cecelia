use std::fmt::Display;

use colored::Colorize;

use crate::{lexer::TokenType, position::Position, r#type::Type};

pub struct Error {
    pub error_type: ErrorType,
    pub position: Position,
}

pub enum ErrorType {
    UnexpectedEOF,

    UnexpectedCharacter(char),
    ExpectedCharacter(char),
    InvalidNumber(String),

    UnexpectedToken(TokenType),
    ExpectedToken(TokenType),
    ExpectedAnyIdentifier,
    ExpectedType(Type, Type),

    UnableToParseStatement(TokenType),
    UnableToParseExpression(TokenType),
    UnableToResolveType(String),

    TypeMismatch(Type, Type),

    VariableAlreadyDeclared(String),
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorType::UnexpectedCharacter(char) => {
                write!(f, "Unexpected character: {}", char)
            },

            ErrorType::InvalidNumber(string) => {
                write!(f, "Invalid number: `{}`", string)
            },

            ErrorType::ExpectedCharacter(char) => {
                write!(f, "Expected character: {}", char)
            },

            ErrorType::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {:?}", token)
            },

            ErrorType::ExpectedToken(token) => {
                write!(f, "Expected token: {:?}", token)
            },

            ErrorType::ExpectedAnyIdentifier => {
                write!(f, "Expected any identifier")
            },

            ErrorType::ExpectedType(expected, actual) => {
                write!(f, "Expected type: `{:?}` but got `{:?}`", expected, actual)
            },

            ErrorType::TypeMismatch(expected, actual) => {
                write!(f, "Type mismatch: `{:?}` and `{:?}`", expected, actual)
            },

            ErrorType::UnableToParseStatement(token) => {
                write!(f, "Unable to parse statement: {:?}", token)
            },

            ErrorType::UnableToParseExpression(token) => {
                write!(f, "Unable to parse expression: {:?}", token)
            },

            ErrorType::UnableToResolveType(type_name) => {
                write!(f, "Unable to resolve type: `{}`", type_name)
            },

            ErrorType::UnexpectedEOF => write!(f, "Unexpected EOF"),

            ErrorType::VariableAlreadyDeclared(name) => {
                write!(f, "Variable already declared: `{}`", name)
            },
        }
    }
}

impl Error {
    pub fn new(error_type: ErrorType, position: Position) -> Self {
        Self { error_type, position }
    }

    pub fn print_error(&self, input: String) {
        let line = input.lines().nth(self.position.y).unwrap();
        let line_number = self.position.y + 1;
        let column = self.position.x - 1;

        eprintln!(
            "{}",
            format!("Error at line {} column {}: ", line_number, self.position.x).red().bold()
        );

        eprintln!("{}", line.white());
        eprintln!("{}", format!("{}^", " ".repeat(column)).bold());
        eprintln!("{}{}\n", " ".repeat(column), format!("{}", self.error_type).bold())
    }
}
