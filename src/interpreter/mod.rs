pub mod value;

use std::collections::HashMap;

use crate::{
    ast::{Expression, Identifier, Statement},
    resolver::{ExpressionVisitor, StatementVisitor},
    Error, ErrorType,
};
use colored::Colorize;
use value::*;

#[derive(Default)]
pub struct Interpreter {
    variables: HashMap<Identifier, Value>,
}

impl Interpreter {
    pub fn print_variables(&self) {
        println!("Variables:");
        for (identifier, value) in &self.variables {
            println!("  - {}: {:?}", identifier.name, value);
        }
    }
}

impl StatementVisitor<()> for Interpreter {
    fn visit_statement(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Let { identifier, value, .. } => {
                // If there is already a variable with the same name, throw an error.
                if self.variables.contains_key(&identifier) {
                    return Err(Error::new(
                        ErrorType::VariableAlreadyDeclared(identifier.name),
                        identifier.position,
                    ));
                }

                // Evaluate the value of the expression.
                let value = self.visit_expression(value)?;
                self.variables.insert(identifier, value);

                Ok(())
            },

            Statement::Return { .. } => {
                println!(
                    "{}",
                    "TODO: Implement interpreter for return statements".to_string().yellow()
                );

                Ok(())
            },
        }
    }
}

impl ExpressionVisitor<Value> for Interpreter {
    fn visit_expression(&mut self, expression: Expression) -> Result<Value, Error> {
        let value = match expression {
            Expression::IntegerLiteral(value) => Value::Integer(value),
            Expression::StringLiteral(value) => Value::String(value),
            Expression::BinaryOperation {
                left,
                operator,
                position,
                right,
                ..
            } => {
                // Evaluate the left and right expressions.
                let left = self.visit_expression(*left)?;
                let right = self.visit_expression(*right)?;

                // If the binary operation fails, the types are incompatible.
                left.binary_operation(operator, right.clone()).ok_or(Error::new(
                    ErrorType::InvalidBinaryOperation(left, operator, right),
                    position,
                ))?
            },

            _ => todo!("Implement interpreter for {:?} expressions", expression),
        };

        Ok(value)
    }
}
