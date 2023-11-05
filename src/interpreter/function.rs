use super::value::Value;
use crate::r#type::Type;

pub trait Function {
    /// Call the function with the given arguments.
    /// The number of arguments will match the number of arguments returned by `arguments()`,
    /// and the types of the arguments will also match.
    fn call(&self, arguments: Vec<Value>) -> Value;

    /// The return type of the function.
    /// If the function returns nothing, the return type should be `Type::Void`.
    fn return_type(&self) -> Type;

    /// An empty vector means that the function takes no arguments.
    /// If the function takes a variable number of arguments, the last type should be `Type::Void`.
    fn arguments(&self) -> Vec<Type>;
}

/// A collection of built-in functions.
/// This is used by the interpreter to call built-in functions.
#[derive(Default)]
pub struct BuiltinFunctions;

impl BuiltinFunctions {
    /// Get a built-in function by name.
    /// If the function does not exist, returns `None`.
    pub fn get(&self, name: &str) -> Option<&dyn Function> {
        match name {
            "print" => Some(&PrintFunction),
            _ => None,
        }
    }
}

pub struct PrintFunction;

/// The built-in `print` function.
/// This function takes a value of any type, and returns nothing.
impl Function for PrintFunction {
    fn call(&self, arguments: Vec<Value>) -> Value {
        for argument in arguments {
            println!("{}", argument);
        }

        Value::Void
    }

    fn return_type(&self) -> Type {
        Type::Void
    }

    fn arguments(&self) -> Vec<Type> {
        vec![Type::Any]
    }
}
