use crate::position::Position;

use super::{r#type::Type, value::Value};

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub position: Position,
}

impl Node {
    pub fn new(node_type: NodeType, position: Position) -> Self {
        Self {
            node_type,
            position,
        }
    }
}

#[derive(Debug)]
pub enum NodeType {
    LetStatement {
        identifier: String,
        value_type: Type,
        value: Value,
    },
    ReturnStatement {
        value_type: Type,
        value: Value,
    },
}
