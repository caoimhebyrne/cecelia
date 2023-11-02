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
                '=' => Token::new(TokenType::Equals, self.position()),
                '+' => Token::new(TokenType::Plus, self.position()),
                '-' => Token::new(TokenType::Minus, self.position()),
                '*' => Token::new(TokenType::Asterisk, self.position()),
                ':' => Token::new(TokenType::Colon, self.position()),

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
                        Token::new(TokenType::Slash, self.position())
                    }
                }

                ' ' => continue,

                _ => {
                    if char.is_alphabetic() {
                        self.parse_identifier(char)
                    } else if char.is_numeric() {
                        self.parse_number(char)?
                    } else {
                        return Err(LexerError::new(
                            LexerErrorType::UnexpectedCharacter(char),
                            self.position(),
                        ));
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

        Token::new(token_type, self.position())
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

        match number_string.parse::<i32>() {
            Ok(value) => Ok(Token::new(
                TokenType::IntegerLiteral(value),
                self.position(),
            )),
            Err(_) => Err(LexerError::new(
                LexerErrorType::InvalidNumber(number_string),
                self.position(),
            )),
        }
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
}
