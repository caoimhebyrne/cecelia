use std::fmt::Display;

use colored::Colorize;

use crate::lexer::Position;

pub struct LexerError {
    pub lexer_error_type: LexerErrorType,
    pub position: Position,
}

pub enum LexerErrorType {
    UnexpectedCharacter(char),
}

impl Display for LexerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErrorType::UnexpectedCharacter(char) => {
                write!(f, "Unexpected character: `{}`", char)
            }
        }
    }
}

impl LexerError {
    pub fn new(lexer_error_type: LexerErrorType, position: Position) -> Self {
        Self {
            lexer_error_type,
            position,
        }
    }

    pub fn print_error(&self, input: String) {
        let line = input.lines().nth(self.position.y).unwrap();
        let line_number = self.position.y + 1;
        let column = self.position.x - 1;

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
            format!("{}", self.lexer_error_type).bold()
        )
    }
}
