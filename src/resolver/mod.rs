pub use crate::visitor::*;

use crate::{
    ast::{Expression, Identifier, Statement},
    position::Position,
    r#type::Type,
    Error, ErrorType,
};

/// Resolves any unresolved or uninferred types.
#[derive(Debug, Default)]
pub struct TypeResolver {}

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
        }
    }
}

impl ExpressionVisitor<Expression> for TypeResolver {
    fn visit_expression(&mut self, expression: Expression) -> Result<Expression, Error> {
        match expression {
            Expression::IntegerLiteral(_) => Ok(expression),
            Expression::StringLiteral(_) => Ok(expression),
            Expression::Identifier(r#type, identifier) => {
                // If the type is unresolved, and can be resolved, resolve it.
                let resolved_type = Self::resolve_type(r#type.clone(), identifier.position)?;
                Ok(Expression::Identifier(resolved_type, identifier.clone()))
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

        // If the type is unresolvable, infer it from the value.
        if let Type::Unresolved(None) = resolved_type {
            resolved_type = value.r#type();
        }

        // Ensure that the type of the value matches the type of the variable.
        if resolved_type != value.r#type() {
            return Err(Error::new(
                ErrorType::TypeMismatch(resolved_type, value.r#type()),
                position,
            ));
        }

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
