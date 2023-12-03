use std::fmt;

pub enum CompilerError {
    UndefinedVariable(String),
    TypeMismatch(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            CompilerError::TypeMismatch(msg) => write!(f, "Type mismatch: {}", msg),
        }
    }
}
