use crate::position::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Equals,   // =
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    Colon,    // :

    Keyword(Keyword),
    Identifier(String),
    StringLiteral(String),
    IntegerLiteral(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Let,
    Return,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, position: Position) -> Self {
        Self { token_type, position }
    }
}
