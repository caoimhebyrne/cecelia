#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    IntegerLiteral(i32),
    StringLiteral(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionType {
    Unresolved(String),
    String,
    Integer,
}
