use std::vec::Vec;

pub struct Program {
    pub statements: Vec<Statement>,
}
pub enum Statement {
    Expr(Box<Expr>),
    Assign(String, Box<Expr>),
    Return(Box<Expr>),
    If(Box<Expr>, Box<Statement>, Box<Option<Statement>>),
    Block(Vec<Statement>),
    While(Box<Expr>, Box<Statement>),
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
}

pub enum Atom {
    Number(i32),
    Expr(Box<Expr>),
    Variable(String),
}
