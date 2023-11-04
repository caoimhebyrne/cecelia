use crate::position::Position;

use super::expression::{Expression, ExpressionType};

#[derive(Debug)]
pub enum Node {
    // let x<: Type?> = <Expression>
    LetStatement(Position, Identifier, Option<ExpressionType>, Expression),

    // return <Expression?>
    ReturnStatement(Position, Option<Expression>),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub position: Position,
    pub value: String,
}

impl Identifier {
    pub fn new(position: Position, value: String) -> Self {
        Self { position, value }
    }
}
