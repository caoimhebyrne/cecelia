pub mod node;
pub use node::*;

use crate::{
    lexer::{Keyword, Token, TokenType},
    position::Position,
    r#type::Type,
    stream::Stream,
    Error, ErrorType,
};
use std::convert::Into;

pub struct AST {
    tokens: Stream<Token>,
}

impl AST {
    /// Creates a new AST from a list of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: Stream::new(tokens),
        }
    }

    /// Parses the [tokens] into a list of statements.
    pub fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = vec![];

        loop {
            let Some(token) = self.tokens.peek() else {
                break;
            };

            let statement = match token.token_type {
                TokenType::Keyword(Keyword::Let) => {
                    self.tokens.consume();
                    self.parse_let_statement(token.position)?
                },

                TokenType::Keyword(Keyword::Return) => {
                    self.tokens.consume();
                    self.parse_return_statement(token.position)?
                },

                _ => {
                    let expression = self.parse_expression(token.position)?;
                    Statement::Expression(expression)
                },
            };

            statements.push(statement);
        }

        Ok(statements)
    }

    /// Parses an expression.
    /// <expression> ::= <identifier> | <literal>
    fn parse_expression(&mut self, last_position: Position) -> Result<Expression, Error> {
        // We don't consume this as the caller may be able to parse it as a statement in the case that it is not an expression.
        let Some(token) = self.tokens.consume() else {
            return Err(Error::new(ErrorType::UnexpectedEOF, last_position));
        };

        let expression = match token.token_type {
            TokenType::Identifier(value) => {
                let identifier = Identifier::new(value, token.position);

                // We need to check if the next token is an open parenthesis.
                let next_token = self.tokens.peek();

                if let Some(Token {
                    token_type: TokenType::OpenParenthesis,
                    ..
                }) = next_token
                {
                    // This is a function call.
                    self.tokens.consume();

                    let mut arguments = vec![];
                    loop {
                        let next_token = self.tokens.peek();

                        if let Some(Token {
                            token_type: TokenType::CloseParenthesis,
                            ..
                        }) = next_token
                        {
                            self.tokens.consume();
                            break;
                        }

                        let argument = self.parse_expression(token.position)?;
                        arguments.push(argument);

                        let next_token = self.tokens.peek();

                        if let Some(Token {
                            token_type: TokenType::Comma,
                            ..
                        }) = next_token
                        {
                            self.tokens.consume();
                        } else {
                            continue;
                        }
                    }

                    return Ok(Expression::FunctionCall {
                        identifier,
                        arguments,
                        r#type: Type::default(),
                    });
                }

                Expression::Identifier(Type::Unresolved(None), identifier)
            },

            TokenType::IntegerLiteral(value) => Expression::IntegerLiteral(value),

            TokenType::StringLiteral(value) => Expression::StringLiteral(value),

            // Unable to parse the token as an expression.
            _ => {
                self.tokens.unconsume();
                return Err(Error::new(ErrorType::UnexpectedToken(token.token_type), token.position));
            },
        };

        // If the next token is an operator, this is a binary operation expression.
        let next_token = self.tokens.peek();
        if let Some(operator) = next_token.and_then(|token| token.token_type.into()) {
            self.tokens.consume();

            let right_expression = self.parse_expression(token.position)?;

            return Ok(Expression::BinaryOperation {
                left: Box::new(expression),
                right: Box::new(right_expression),
                r#type: Type::default(),
                position: token.position,
                operator,
            });
        }

        Ok(expression)
    }

    /// Parses a let statement.
    /// let <identifier><: Type?> = <expression>
    fn parse_let_statement(&mut self, last_position: Position) -> Result<Statement, Error> {
        let identifier = self.parse_identifier(last_position)?;

        // The next token can either be a colon or an equals sign.
        let token = self.tokens.consume().ok_or_else(|| Error::new(ErrorType::UnexpectedEOF, identifier.position))?;
        let r#type = match token.token_type {
            TokenType::Colon => {
                let value = self.parse_type_identifier(token.position).unwrap_or(Type::Unresolved(None));

                // We must also ensure that the next token is an equals sign.
                self.tokens
                    .consume()
                    .ok_or_else(|| Error::new(ErrorType::ExpectedToken(TokenType::Equals), token.position))?;

                value
            },

            // Nothing else to parse.
            TokenType::Equals => Type::Unresolved(None),

            // Invalid token.
            _ => return Err(Error::new(ErrorType::UnexpectedToken(token.token_type), token.position)),
        };

        let value = self.parse_expression(token.position)?;

        Ok(Statement::Let {
            identifier,
            value,
            r#type,
            position: token.position,
        })
    }

    /// Parses a return statement.
    /// return <expression?>
    fn parse_return_statement(&mut self, last_position: Position) -> Result<Statement, Error> {
        let value = self.parse_expression(last_position).ok();

        Ok(Statement::Return {
            value,
            position: last_position,
        })
    }

    /// Parses an identifier.
    fn parse_identifier(&mut self, last_position: Position) -> Result<Identifier, Error> {
        let token = self.tokens.consume().ok_or_else(|| Error::new(ErrorType::ExpectedAnyIdentifier, last_position))?;

        if let TokenType::Identifier(name) = token.token_type {
            Ok(Identifier {
                name,
                position: token.position,
            })
        } else {
            Err(Error::new(ErrorType::ExpectedAnyIdentifier, token.position))
        }
    }

    /// Parses a type identifier.
    fn parse_type_identifier(&mut self, last_position: Position) -> Result<Type, Error> {
        let token = self.tokens.consume().ok_or_else(|| Error::new(ErrorType::UnexpectedEOF, last_position))?;

        if let TokenType::Identifier(name) = token.token_type {
            Ok(match name.as_str() {
                "Integer" => Type::Integer,
                "String" => Type::String,
                _ => Type::Unresolved(Some(name)),
            })
        } else {
            Err(Error::new(ErrorType::ExpectedAnyIdentifier, token.position))
        }
    }
}
