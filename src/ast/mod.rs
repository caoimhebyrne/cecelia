pub mod node;
pub use node::*;

use crate::{
    lexer::{Keyword, Token, TokenType},
    position::Position,
    r#type::Type,
    stream::Stream,
    Error, ErrorType,
};

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
            let Some(token) = self.tokens.consume() else {
                break;
            };

            let statement = match token.token_type {
                TokenType::Keyword(Keyword::Let) => self.parse_let_statement(token.position)?,
                TokenType::Keyword(Keyword::Return) => self.parse_return_statement(token.position)?,

                _ => {
                    return Err(Error::new(
                        ErrorType::UnableToParseStatement(token.token_type),
                        token.position,
                    ))
                },
            };

            statements.push(statement);
        }

        Ok(statements)
    }

    /// Parses an expression.
    /// <expression> ::= <identifier> | <literal>
    fn parse_expression(&mut self, last_position: Position) -> Result<Expression, Error> {
        let Some(token) = self.tokens.consume() else {
            return Err(Error::new(ErrorType::UnexpectedEOF, last_position));
        };

        let expression = match token.token_type {
            TokenType::Identifier(value) => Expression::Identifier(Identifier::new(value, token.position)),
            TokenType::IntegerLiteral(value) => Expression::IntegerLiteral(value),
            TokenType::StringLiteral(value) => Expression::StringLiteral(value),

            // Unable to parse the token as an expression.
            _ => return Err(Error::new(ErrorType::UnexpectedToken(token.token_type), token.position)),
        };

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
            position: last_position,
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
