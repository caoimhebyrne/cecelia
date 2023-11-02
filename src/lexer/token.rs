#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
        Self {
            token_type,
            position,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn next_char(&mut self) {
        self.x += 1;
    }

    pub fn next_line(&mut self) {
        self.x = 0;
        self.y += 1;
    }
}
