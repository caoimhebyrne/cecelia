use std::fmt::Display;

use colored::Colorize;

use crate::position::Position;

pub struct Error {
    pub error_type: ErrorType,
    pub position: Position,
}

pub enum ErrorType {
    UnexpectedCharacter(char),
    ExpectedCharacter(char),
    InvalidNumber(String),
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
