mod error;
mod expression;
mod node;

use crate::{
    lexer::token::{Keyword, Token, TokenType},
    position::Position,
    stream::Stream,
};

use error::{ASTError, ASTErrorType};
use expression::{Expression, ExpressionType};
use node::{Identifier, Node};

pub struct AST {
    tokens: Stream<Token>,
}

impl AST {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens.into() }
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

                _ => return ASTError::new(ASTErrorType::UnableToParse(token.token_type), position).into(),
            };

            nodes.push(node);
        }

        Ok(nodes)
    }

    pub fn parse_expression(&mut self, position: Position) -> Result<Expression, ASTError> {
        let Some(token) = self.tokens.consume() else {
            return ASTError::new(ASTErrorType::UnexpectedEOF, position).into();
        };

        let value = match token.token_type {
            TokenType::IntegerLiteral(value) => Expression::IntegerLiteral(value),
            TokenType::StringLiteral(value) => Expression::StringLiteral(value),

            _ => return ASTError::new(ASTErrorType::UnableToParseValue(token.token_type), token.position).into(),
        };

        Ok(value)
    }

    // let <identifier>: <type?> = <expression>
    pub fn parse_let_statement(&mut self, position: Position) -> Result<Node, ASTError> {
        let identifier = self.expect_identifier(position.clone())?;

        let value_type = match self.tokens.peek() {
            Some(token) if token.token_type == TokenType::Colon => {
                self.tokens.consume();
                self.parse_type_identifier(token.position).ok()
            },
            _ => None,
        };

        let equals_token = self.expect_equals(identifier.position.clone())?;
        let value = self.parse_expression(equals_token.position)?;

        Ok(Node::LetStatement(position, identifier, value_type, value))
    }

    fn parse_return_statement(&mut self, position: Position) -> Result<Node, ASTError> {
        let value = self.parse_expression(position.clone()).ok();
        Ok(Node::ReturnStatement(position, value))
    }

    fn parse_type_identifier(&mut self, position: Position) -> Result<ExpressionType, ASTError> {
        let Some(token) = self.tokens.consume() else {
            return ASTError::new(
                ASTErrorType::ExpectedToken(TokenType::Identifier("any".into())),
                position,
            )
            .into();
        };

        let identifier = match token.token_type {
            TokenType::Identifier(identifier) => identifier,
            _ => {
                return ASTError::new(
                    ASTErrorType::ExpectedToken(TokenType::Identifier("any".into())),
                    token.position.as_previous(),
                )
                .into();
            },
        };

        Ok(match identifier.as_str() {
            "String" => ExpressionType::String,
            "Integer" => ExpressionType::Integer,
            _ => ExpressionType::Unresolved(identifier),
        })
    }

    fn expect_equals(&mut self, position: Position) -> Result<Token, ASTError> {
        let Some(token) = self.tokens.consume() else {
            return ASTError::new(ASTErrorType::ExpectedToken(TokenType::Equals), position).into();
        };

        if token.token_type != TokenType::Equals {
            return ASTError::new(
                ASTErrorType::ExpectedTokenButGot(TokenType::Equals, token.token_type),
                token.position,
            )
            .into();
        }

        Ok(token)
    }

    fn expect_identifier(&mut self, position: Position) -> Result<Identifier, ASTError> {
        let Some(token) = self.tokens.consume() else {
            return ASTError::new(
                ASTErrorType::ExpectedToken(TokenType::Identifier("any".into())),
                position,
            )
            .into();
        };

        let value = match token.token_type {
            TokenType::Identifier(identifier) => identifier,
            _ => {
                return ASTError::new(
                    ASTErrorType::ExpectedTokenButGot(TokenType::Identifier("any".into()), token.token_type),
                    token.position,
                )
                .into()
            },
        };

        Ok(Identifier::new(token.position, value))
    }
}
