use std::fmt;
use std::vec::Vec;
pub struct Program {
    pub program_units: Vec<ProgramUnit>,
}

#[derive(Debug)]
pub enum ProgramUnit {
    FuncDef(DataType, String, Vec<(DataType, String)>, Box<Statement>),
    Statement(Box<Statement>),
}

#[derive(Debug)]
pub enum Statement {
    Expr(Box<Expr>),
    Assign(String, Box<Expr>),
    AssignPointer(u32, String, Box<Expr>),
    Return(Box<Expr>),
    If(Box<Expr>, Box<Statement>, Box<Option<Statement>>),
    Block(Vec<Statement>),
    While(Box<Expr>, Box<Statement>),
    For(
        Box<Option<Statement>>,
        Box<Option<Expr>>,
        Box<Option<Statement>>,
        Box<Statement>,
    ),
    Break,
    VarDef(DataType, String),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DataType {
    Primitive(PrimitiveType),
    Pointer(u32, Box<DataType>),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PrimitiveType {
    Int,
    Void,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Primitive(primitive_type) => write!(f, "{}", primitive_type),
            DataType::Pointer(depth, data_type) => {
                write!(f, "{}", data_type)?;
                for _ in 0..*depth {
                    write!(f, "*")?;
                }
                Ok(())
            }
        }
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveType::Int => write!(f, "int"),
            PrimitiveType::Void => write!(f, "void"),
        }
    }
}

pub fn int() -> DataType {
    DataType::Primitive(PrimitiveType::Int)
}

pub fn void() -> DataType {
    DataType::Primitive(PrimitiveType::Void)
}

pub fn pointer(depth: u32, data_type: DataType) -> DataType {
    DataType::Pointer(depth, Box::new(data_type))
}

#[derive(Debug)]
pub enum Expr {
    ArithExpr(Box<ArithExpr>),
    Equal(Box<ArithExpr>, Box<ArithExpr>),
    NotEqual(Box<ArithExpr>, Box<ArithExpr>),
    Less(Box<ArithExpr>, Box<ArithExpr>),
    LessOrEqual(Box<ArithExpr>, Box<ArithExpr>),
}

#[derive(Debug)]
pub enum ArithExpr {
    Factor(Box<Factor>),
    Add(Box<ArithExpr>, Box<Factor>),
    Sub(Box<ArithExpr>, Box<Factor>),
}

#[derive(Debug)]
pub enum Factor {
    Unary(Box<Unary>),
    Mul(Box<Factor>, Box<Unary>),
    Div(Box<Factor>, Box<Unary>),
}

#[derive(Debug)]
pub enum Unary {
    Atom(Box<Atom>),
    Neg(Box<Atom>),
    PointerDeref(Box<Atom>),
}

#[derive(Debug)]
pub enum Atom {
    Number(i32),
    Expr(Box<Expr>),
    Variable(String),
    AddressOf(String),
    FunctionCall(String, Vec<Expr>),
}
