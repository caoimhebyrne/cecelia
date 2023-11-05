pub mod function;
pub mod value;

use std::collections::HashMap;

use crate::{
    ast::{Expression, Identifier, Statement},
    resolver::{ExpressionVisitor, StatementVisitor},
    Error, ErrorType,
};
use function::*;
use value::*;

#[derive(Default)]
pub struct Interpreter {
    variables: HashMap<Identifier, Value>,
    builtin_functions: BuiltinFunctions,
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

            Statement::Return { value, position } => {
                let value = value.map(|it| self.visit_expression(it)).transpose()?;
                Err(Error::new(ErrorType::Return(value), position))
            },

            Statement::Expression(expression) => {
                self.visit_expression(expression)?;
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

            Expression::Identifier(.., identifier) => {
                // Look up the variable in the variables map.
                self.variables
                    .get(&identifier)
                    .ok_or(Error::new(
                        ErrorType::UnableToResolveType(identifier.clone().name),
                        identifier.position,
                    ))?
                    .clone()
            },

            Expression::FunctionCall {
                identifier, arguments, ..
            } => {
                // Evaluate the values to be passed to the function.
                let mut values = Vec::new();
                for argument in &arguments {
                    values.push(self.visit_expression(argument.clone())?);
                }

                // Look up the function in the functions map.
                let function = self.builtin_functions.get(&identifier.name).ok_or(Error::new(
                    ErrorType::UnknownFunction(identifier.name.clone()),
                    identifier.position,
                ))?;

                // At this point, the typechecker should have ensured that the types of the arguments match the types of the parameters.
                function.call(values)
            },
        };

        Ok(value)
    }
}
