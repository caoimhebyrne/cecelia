use crate::{lexer::TokenType, position::Position, r#type::Type};

/// The different types of statements that can be parsed.
#[derive(Debug, Clone)]
pub enum Statement {
    /// A statement that assigns a value to a variable.
    Let {
        /// The name of the variable.
        identifier: Identifier,

        /// The value to assign to the variable.
        value: Expression,

        /// The declared type of the variable.
        r#type: Type,

        /// The position of the let statement in the source code.
        position: Position,
    },

    Return {
        /// The value to return.
        value: Option<Expression>,

        /// The position of the return statement in the source code.
        position: Position,
    },
}

/// Represents the name of a variable, function, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    /// The name of the identifier.
    pub name: String,

    // The position of the identifier in the source code.
    pub position: Position,
}

impl Identifier {
    /// Creates a new identifier.
    pub fn new(name: String, position: Position) -> Self {
        Self { name, position }
    }
}

/// Represents an expression.
#[derive(Debug, Clone)]
pub enum Expression {
    /// An integer literal.
    IntegerLiteral(i32),

    /// A string literal.
    StringLiteral(String),

    /// A variable.
    Identifier(Type, Identifier),

    /// A binary operation between two expressions.
    BinaryOperation {
        /// The left-hand side of the binary operation.
        left: Box<Expression>,

        /// The operator of the binary operation.
        operator: Operator,

        /// The position of the operator in the source code.
        position: Position,

        /// The right-hand side of the binary operation.
        right: Box<Expression>,

        /// The type of the binary operation.
        r#type: Type,
    },
}

impl Expression {
    /// Returns the type of the expression.
    pub fn r#type(&self) -> Type {
        match self {
            Self::IntegerLiteral(_) => Type::Integer,
            Self::StringLiteral(_) => Type::String,
            Self::Identifier(r#type, _) => r#type.clone(),
            Self::BinaryOperation { left, .. } => left.r#type(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl From<TokenType> for Option<Operator> {
    fn from(val: TokenType) -> Self {
        let operator = match val {
            TokenType::Plus => Operator::Add,
            TokenType::Minus => Operator::Subtract,
            TokenType::Asterisk => Operator::Multiply,
            TokenType::Slash => Operator::Divide,
            _ => return None,
        };

        Some(operator)
    }
}
