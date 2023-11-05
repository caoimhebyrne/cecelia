use crate::{
    ast::{Expression, Statement},
    Error,
};

pub trait StatementVisitor<T> {
    /// Visits a list of statements.
    /// This is a default implementation that calls `visit_statement` for each statement.
    fn visit_statements(&mut self, statements: Vec<Statement>) -> Result<Vec<T>, Error> {
        statements.iter().map(|statement| self.visit_statement(statement.clone())).collect()
    }

    /// Visits a statement.
    fn visit_statement(&mut self, statement: Statement) -> Result<T, Error>;
}

pub trait ExpressionVisitor<T> {
    /// Visits an expression
    fn visit_expression(&mut self, expression: &Expression) -> Result<T, Error>;
}
