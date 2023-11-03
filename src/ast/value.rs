#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    IntegerLiteral(i32),
    StringLiteral(String),
    None,
}
