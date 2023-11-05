use std::collections::HashMap;

use colored::Colorize;

pub use crate::visitor::*;

use crate::{
    ast::{Expression, Identifier, Statement},
    interpreter::function::BuiltinFunctions,
    position::Position,
    r#type::Type,
    Error, ErrorType,
};

/// Resolves any unresolved or uninferred types.
#[derive(Default)]
pub struct TypeResolver {
    variables: HashMap<Identifier, Type>,
    builtin_functions: BuiltinFunctions,
}

impl StatementVisitor<Statement> for TypeResolver {
    fn visit_statement(&mut self, statement: Statement) -> Result<Statement, Error> {
        match statement {
            Statement::Let {
                identifier,
                value,
                r#type,
                position,
            } => self.visit_let_statement(identifier, value, r#type, position),

            Statement::Return { value, position } => {
                let value = value.map(|value| self.visit_expression(value)).transpose()?;
                Ok(Statement::Return { value, position })
            },

            Statement::Expression(expression) => {
                let expression = self.visit_expression(expression)?;
                Ok(Statement::Expression(expression))
            },
        }
    }
}

impl ExpressionVisitor<Expression> for TypeResolver {
    fn visit_expression(&mut self, expression: Expression) -> Result<Expression, Error> {
        match expression {
            Expression::IntegerLiteral(_) => Ok(expression),
            Expression::StringLiteral(_) => Ok(expression),

            Expression::BinaryOperation {
                left,
                right,
                position,
                operator,
                ..
            } => {
                // If the type is unresolved, and can be resolved, resolve it.
                let left_type = self.visit_expression(*left.clone())?.r#type();
                let right_type = self.visit_expression(*right.clone())?.r#type();

                // Ensure that the type of the value matches the type of the variable.
                if left_type != right_type {
                    return Err(Error::new(ErrorType::TypeMismatch(left_type, right_type), position));
                }

                Ok(Expression::BinaryOperation {
                    left,
                    operator,
                    position,
                    right,
                    r#type: left_type,
                })
            },

            Expression::Identifier(.., identifier) => {
                // Identifiers have no type attached to them, so we need to look up the type in the variables map.
                let resolved_type = self
                    .variables
                    .get(&identifier)
                    .ok_or(Error::new(
                        ErrorType::UnknownVariable(identifier.clone().name),
                        identifier.position,
                    ))?
                    .clone();

                println!(
                    "{}: resolved type of `{}` to `{:?}`",
                    "info(resolver)".blue(),
                    identifier.name,
                    resolved_type
                );

                Ok(Expression::Identifier(resolved_type, identifier))
            },

            Expression::FunctionCall {
                identifier, arguments, ..
            } => {
                // Resolve the types of any expressions passed as arguments.
                let mut expressions = Vec::new();
                for argument in arguments {
                    expressions.push(self.visit_expression(argument)?);
                }

                // TODO: Add support for code-defined functions.
                let function = self.builtin_functions.get(&identifier.name).ok_or(Error::new(
                    ErrorType::UnknownFunction(identifier.name.clone()),
                    identifier.position,
                ))?;

                // Ensure that the number of arguments matches the number of arguments the function takes.
                if expressions.len() != function.arguments().len() {
                    return Err(Error::new(
                        ErrorType::InvalidNumberOfArguments(function.arguments().len(), expressions.len()),
                        identifier.position,
                    ));
                }

                // Ensure that the types of the arguments match the types of the arguments the function takes.
                for (index, expression) in expressions.iter().enumerate() {
                    // This is safe because we already checked that the number of arguments matches the number of arguments the function takes.
                    let expected_type = function.arguments().get(index).unwrap().clone();
                    let actual_type = expression.r#type();

                    // If the expected type is `Any`, then we can skip the type check.
                    if expected_type == Type::Any {
                        continue;
                    }

                    if actual_type != expected_type {
                        return Err(Error::new(
                            ErrorType::TypeMismatch(actual_type, expected_type),
                            identifier.position,
                        ));
                    }
                }

                Ok(Expression::FunctionCall {
                    identifier,
                    arguments: expressions,
                    r#type: function.return_type(),
                })
            },
        }
    }
}

impl TypeResolver {
    fn visit_let_statement(
        &mut self,
        identifier: Identifier,
        value: Expression,
        r#type: Type,
        position: Position,
    ) -> Result<Statement, Error> {
        // First, resolve the type of the value.
        let value = self.visit_expression(value)?;

        // If the type is unresolved, and can be resolved, resolve it.
        let mut resolved_type = Self::resolve_type(r#type, position)?;

        // If the resolved type still needs to be inferred, infer it from the value type.
        if let Type::Unresolved(None) = resolved_type {
            // If the value type still needs to be inferred, then we can't infer the type of the variable.
            if let Type::Unresolved(None) = value.r#type() {
                return Err(Error::new(ErrorType::UnableToInferType, position));
            }

            resolved_type = value.r#type();
        }

        // Ensure that the type of the value matches the type of the variable.
        if resolved_type != value.r#type() {
            return Err(Error::new(
                ErrorType::TypeMismatch(resolved_type, value.r#type()),
                position,
            ));
        }

        self.variables.insert(identifier.clone(), resolved_type.clone());

        Ok(Statement::Let {
            identifier,
            value,
            r#type: resolved_type,
            position,
        })
    }

    fn resolve_type(r#type: Type, last_position: Position) -> Result<Type, Error> {
        // If the type is unresolved, and can be resolved, resolve it.
        if let Type::Unresolved(Some(type_name)) = r#type {
            Err(Error::new(ErrorType::UnableToResolveType(type_name), last_position))
        } else {
            // Otherwise, return the type as is.
            Ok(r#type)
        }
    }
}
