use std::vec::Vec;

pub struct Program {
    pub program_units: Vec<ProgramUnit>,
}

pub enum ProgramUnit {
    FuncDef(DataType, String, Vec<(DataType, String)>, Box<Statement>),
    Statement(Box<Statement>),
}

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

#[derive(Clone)]
pub enum DataType {
    Primitive(PrimitiveType),
    Pointer(u32, Box<DataType>),
}

#[derive(Clone)]
pub enum PrimitiveType {
    Int,
}

pub enum Expr {
    ArithExpr(Box<ArithExpr>),
    Equal(Box<ArithExpr>, Box<ArithExpr>),
    NotEqual(Box<ArithExpr>, Box<ArithExpr>),
    Less(Box<ArithExpr>, Box<ArithExpr>),
    LessOrEqual(Box<ArithExpr>, Box<ArithExpr>),
}
pub enum ArithExpr {
    Factor(Box<Factor>),
    Add(Box<ArithExpr>, Box<Factor>),
    Sub(Box<ArithExpr>, Box<Factor>),
}

pub enum Factor {
    Unary(Box<Unary>),
    Mul(Box<Factor>, Box<Unary>),
    Div(Box<Factor>, Box<Unary>),
}

pub enum Unary {
    Atom(Box<Atom>),
    Neg(Box<Atom>),
    PointerDeref(Box<Atom>),
}

pub enum Atom {
    Number(i32),
    Expr(Box<Expr>),
    Variable(String),
    AddressOf(String),
    FunctionCall(String, Vec<Expr>),
}
