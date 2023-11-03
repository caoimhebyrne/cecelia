mod error;
mod node;
mod r#type;
mod value;

use crate::{
    lexer::token::{Keyword, Token, TokenType},
    position::Position,
    stream::Stream,
};

use error::ASTError;
use node::Node;
use r#type::Type;
use value::Value;

use self::{error::ASTErrorType, node::NodeType};

pub struct AST {
    tokens: Stream<Token>,
}

impl AST {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, ASTError> {
        let mut nodes = vec![];

        loop {
            let Some(token) = self.tokens.consume() else {
                break;
            };

            let position = token.position;
            let node = match token.token_type {
                TokenType::Keyword(Keyword::Let) => self.parse_let_statement(position)?,
                TokenType::Keyword(Keyword::Return) => self.parse_return_statement(position)?,

                _ => {
                    return Err(ASTError::new(
                        ASTErrorType::UnableToParse(token.token_type),
                        position,
                    ))
                }
            };

            nodes.push(node);
        }

        Ok(nodes)
    }

    pub fn parse_value(&mut self, position: Position) -> Result<Value, ASTError> {
        let Some(token) = self.tokens.consume() else {
            return Err(ASTError::new(ASTErrorType::UnexpectedEOF, position));
        };

        let value = match token.token_type {
            TokenType::IntegerLiteral(value) => Value::IntegerLiteral(value),
            TokenType::StringLiteral(value) => Value::StringLiteral(value),

            _ => {
                return Err(ASTError::new(
                    ASTErrorType::UnableToParseValue(token.token_type),
                    token.position,
                ))
            }
        };

        Ok(value)
    }

    // let <identifier>: <type?> = <expression>
    pub fn parse_let_statement(&mut self, position: Position) -> Result<Node, ASTError> {
        let (identifier, identifier_position) = self.expect_identifier(position.clone())?;

        let value_type = match self.tokens.peek() {
            Some(token) if token.token_type == TokenType::Colon => {
                self.tokens.consume();
                self.parse_type_identifier(token.position)?
            }
            _ => Type::Unresolved(None),
        };

        let equals_token = self.expect_equals(identifier_position)?;
        let value = self.parse_value(equals_token.position)?;

        let node_type = NodeType::LetStatement {
            identifier,
            value_type,
            value,
        };

        Ok(Node::new(node_type, position))
    }

    fn parse_return_statement(&mut self, position: Position) -> Result<Node, ASTError> {
        let value = self.parse_value(position.clone()).unwrap_or(Value::None);
        let node_type = NodeType::ReturnStatement {
            value_type: match value {
                Value::None => Type::None,
                _ => Type::Unresolved(None),
            },
            value,
        };

        Ok(Node::new(node_type, position))
    }

    fn parse_type_identifier(&mut self, position: Position) -> Result<Type, ASTError> {
        let Some(token) = self.tokens.consume() else {
            return Err(ASTError::new(
                ASTErrorType::ExpectedToken(TokenType::Identifier("any".into())),
                position,
            ));
        };

        let identifier = match token.token_type {
            TokenType::Identifier(identifier) => identifier,
            _ => {
                let mut position = token.position.clone();
                position.previous_char();

                return Err(ASTError::new(
                    ASTErrorType::ExpectedToken(TokenType::Identifier("any".into())),
                    position,
                ));
            }
        };

        Ok(match identifier.as_str() {
            "String" => Type::String,
            "Integer" => Type::Integer,
            _ => Type::Unresolved(Some(identifier)),
        })
    }

    fn expect_equals(&mut self, position: Position) -> Result<Token, ASTError> {
        let Some(token) = self.tokens.consume() else {
            return Err(ASTError::new(
                ASTErrorType::ExpectedToken(TokenType::Equals),
                position,
            ));
        };

        if token.token_type != TokenType::Equals {
            return Err(ASTError::new(
                ASTErrorType::ExpectedTokenButGot(TokenType::Equals, token.token_type),
                token.position,
            ));
        }

        Ok(token)
    }

    fn expect_identifier(&mut self, position: Position) -> Result<(String, Position), ASTError> {
        let Some(maybe_identifier) = self.tokens.consume() else {
            return Err(ASTError::new(
                ASTErrorType::ExpectedToken(TokenType::Identifier("any".into())),
                position,
            ));
        };

        let identifier = match maybe_identifier.token_type {
            TokenType::Identifier(identifier) => identifier,
            _ => {
                return Err(ASTError::new(
                    ASTErrorType::ExpectedTokenButGot(
                        TokenType::Identifier("any".into()),
                        maybe_identifier.token_type,
                    ),
                    maybe_identifier.position,
                ))
            }
        };

        Ok((identifier, maybe_identifier.position))
    }
}
