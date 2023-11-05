use crate::{
    ast::{Expression, Statement},
    Error, ErrorType,
};

pub trait StatementVisitor<T> {
    /// Visits a list of statements.
    /// This is a default implementation that calls `visit_statement` for each statement.
    fn visit_statements(&mut self, statements: Vec<Statement>) -> Result<Vec<T>, Error> {
        let mut results = Vec::new();

        for statement in statements {
            let result = self.visit_statement(statement);

            // If the statement is a return statement, stop execution.
            if let Err(Error {
                error_type: ErrorType::Return(_),
                ..
            }) = result
            {
                break;
            }

            results.push(result?);
        }

        Ok(results)
    }

    /// Visits a statement.
    fn visit_statement(&mut self, statement: Statement) -> Result<T, Error>;
}

pub trait ExpressionVisitor<T> {
    /// Visits an expression
    fn visit_expression(&mut self, expression: Expression) -> Result<T, Error>;
}
