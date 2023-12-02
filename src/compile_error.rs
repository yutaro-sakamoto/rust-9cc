use std::fmt;

pub enum CompilerError {
    UndefinedVariable(String),
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
        }
    }
}
