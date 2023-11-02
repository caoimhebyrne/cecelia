pub mod token;
pub use token::*;

use crate::Stream;
use colored::Colorize;

pub struct Lexer {
    input: String,
    stream: Stream<char>,
    line: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.clone(),
            stream: input.chars().collect::<Vec<char>>().into(),
            line: 0,
        }
    }

    pub fn parse(&mut self) -> Option<Vec<Token>> {
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
                        if let Some(token) = self.parse_number(char) {
                            token
                        } else {
                            continue;
                        }
                    } else {
                        self.print_error(format!("Unexpected character: `{}`", char));
                        return None;
                    }
                }
            };

            tokens.push(token);
        }

        Some(tokens)
    }

    fn print_error(&self, message: String) {
        let mut position = self.position();
        position.next_char();

        let line = self.input.lines().nth(position.y).unwrap();
        let line_number = position.y + 1;

        println!(
            "{}",
            format!("Error at line {} column {}: ", line_number, position.x)
                .red()
                .bold()
        );

        println!("{}", line.white());
        println!("{}", format!("{}^", " ".repeat(position.x - 1)).bold());
        println!("{}{}\n", " ".repeat(position.x - 1), message.bold())
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

    fn parse_number(&mut self, char: char) -> Option<Token> {
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
            .ok()
            .map(|value| Token::new(TokenType::IntegerLiteral(value), self.position()))
    }

    fn skip_until(&mut self, until: char) {
        while let Some(char) = self.stream.consume() {
            if char == until {
                break;
            }
        }
    }

    fn position(&self) -> Position {
        Position::new(self.stream.index - 1, self.line)
    }
}
