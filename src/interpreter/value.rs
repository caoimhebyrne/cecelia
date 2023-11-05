use crate::ast::Operator;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    String(String),
}

impl Value {
    // This should only return None if the types are incompatible.
    pub fn binary_operation(&self, operator: Operator, right: Value) -> Option<Value> {
        match operator {
            Operator::Add => self.add(right),
            Operator::Subtract => self.sub(right),
            Operator::Divide => self.divide(right),
            Operator::Multiply => self.multiply(right),
        }
    }

    // This should only return None if the types are incompatible.
    pub fn add(&self, right: Value) -> Option<Value> {
        let value = match (self, right) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
            (Value::String(left), Value::String(right)) => Value::String(format!("{}{}", left, right)),

            _ => return None,
        };

        Some(value)
    }

    // This should only return None if the types are incompatible.
    pub fn sub(&self, right: Value) -> Option<Value> {
        let value = match (self, right) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
            (Value::String(left), Value::String(right)) => Value::String(format!("{}{}", left, right)),

            _ => return None,
        };

        Some(value)
    }

    // This should only return None if the types are incompatible.
    pub fn divide(&self, right: Value) -> Option<Value> {
        let value = match (self, right) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
            (Value::String(left), Value::String(right)) => Value::String(format!("{}{}", left, right)),

            _ => return None,
        };

        Some(value)
    }

    // This should only return None if the types are incompatible.
    pub fn multiply(&self, right: Value) -> Option<Value> {
        let value = match (self, right) {
            (Value::Integer(left), Value::Integer(right)) => Value::Integer(left + right),
            (Value::String(left), Value::String(right)) => Value::String(format!("{}{}", left, right)),

            _ => return None,
        };

        Some(value)
    }
}
