pub mod error;
pub mod token;

use crate::Stream;
use error::*;
use token::*;

pub struct Lexer {
    stream: Stream<char>,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            stream: input.chars().collect::<Vec<char>>().into(),
            line: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = vec![];

        loop {
            let Some(char) = self.stream.consume() else {
                break;
            };

            let token = match char {
                '=' => self.token(TokenType::Equals),
                '+' => self.token(TokenType::Plus),
                '-' => self.token(TokenType::Minus),
                '*' => self.token(TokenType::Asterisk),
                ':' => self.token(TokenType::Colon),

                '\n' => {
                    self.line += 1;
                    continue;
                }

                '/' => {
                    // Ignore comments...
                    if let Some('/') = self.stream.peek() {
                        self.skip_until('\n');
                        continue;
                    } else {
                        // ... but still emit a slash token for single `/` characters
                        self.token(TokenType::Slash)
                    }
                }

                ' ' => continue,

                _ => {
                    if char.is_alphabetic() {
                        self.parse_identifier(char)
                    } else if char.is_numeric() {
                        self.parse_number(char)?
                    } else {
                        return Err(self.error(LexerErrorType::UnexpectedCharacter(char)));
                    }
                }
            };

            tokens.push(token);
        }

        Ok(tokens)
    }

    fn parse_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first_char);

        loop {
            let Some(char) = self.stream.peek() else {
                break;
            };

            if char.is_alphabetic() {
                self.stream.consume();
                identifier.push(char);
            } else {
                break;
            }
        }

        // Check if identifier is a keyword
        let token_type = match identifier.as_str() {
            "let" => TokenType::Keyword(Keyword::Let),
            "return" => TokenType::Keyword(Keyword::Return),
            _ => TokenType::Identifier(identifier),
        };

        self.token(token_type)
    }

    fn parse_number(&mut self, char: char) -> Result<Token, LexerError> {
        let mut number_string = String::new();
        number_string.push(char);

        loop {
            let Some(char) = self.stream.peek() else {
                break;
            };

            if char.is_numeric() {
                self.stream.consume();
                number_string.push(char);
            } else {
                break;
            }
        }

        number_string
            .parse::<i32>()
            .map(|value| self.token(TokenType::IntegerLiteral(value)))
            .map_err(|_| self.error(LexerErrorType::InvalidNumber(number_string)))
    }

    fn skip_until(&mut self, until: char) {
        while let Some(char) = self.stream.consume() {
            if char == until {
                break;
            }
        }
    }

    fn position(&self) -> Position {
        Position::new(self.stream.index, self.line)
    }

    fn token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.position())
    }

    fn error(&self, error_type: LexerErrorType) -> LexerError {
        LexerError::new(error_type, self.position())
    }
}
