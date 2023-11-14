use std::collections::HashMap;
use std::vec::Vec;

pub struct Program {
    pub statements: Vec<Statement>,
}
pub enum Statement {
    Expr(Box<Expr>),
    Assign(String, Box<Expr>),
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

pub struct MetaInfo {
    pub variable_map: HashMap<String, u32>,
}

impl MetaInfo {
    pub fn new() -> MetaInfo {
        MetaInfo {
            variable_map: HashMap::new(),
        }
    }
}

pub fn print_assembly(program: Box<Program>, meta_info: &mut MetaInfo) {
    let number_of_variables = 16;
    println!(".intel_syntax noprefix");
    println!(".global main\n");
    println!("main:");
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, {}", 8 * number_of_variables);
    for statement in program.statements {
        print_assembly_statement(Box::new(statement), meta_info);
        println!("  pop rax");
    }
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

pub fn print_assembly_statement(statement: Box<Statement>, meta_info: &mut MetaInfo) {
    match *statement {
        Statement::Expr(expr) => print_assembly_expr(expr, meta_info),
        Statement::Assign(left, expr) => {
            print_assembly_lval(left, meta_info);
            print_assembly_expr(expr, meta_info);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
        }
    }
}

fn get_variable_id_and_register_it(lval: &String, meta_info: &mut MetaInfo) -> u32 {
    match meta_info.variable_map.get(lval) {
        Some(id) => *id,
        None => {
            let number_of_variables = meta_info.variable_map.len() as u32;
            meta_info
                .variable_map
                .insert(lval.clone(), number_of_variables);
            number_of_variables
        }
    }
}

fn print_assembly_lval(lval: String, meta_info: &mut MetaInfo) {
    let id = get_variable_id_and_register_it(&lval, meta_info);
    println!("  mov rax, rbp");
    println!("  sub rax, {}", (id + 1) * 8);
    println!("  push rax");
}

fn print_assembly_expr(expr: Box<Expr>, meta_info: &mut MetaInfo) {
    match *expr {
        Expr::ArithExpr(arith_expr) => print_assembly_arith_expr(arith_expr, meta_info),
        Expr::Equal(left, right) => print_compare_instruction("sete", left, right, meta_info),
        Expr::NotEqual(left, right) => print_compare_instruction("setne", left, right, meta_info),
        Expr::Less(left, right) => print_compare_instruction("setl", left, right, meta_info),
        Expr::LessOrEqual(left, right) => {
            print_compare_instruction("setle", left, right, meta_info)
        }
    }
}

fn print_compare_instruction(
    instruction: &str,
    left: Box<ArithExpr>,
    right: Box<ArithExpr>,
    meta_info: &mut MetaInfo,
) {
    print_assembly_arith_expr(left, meta_info);
    print_assembly_arith_expr(right, meta_info);
    println!("  pop rdi");
    println!("  pop rax");
    println!("  cmp rax, rdi");
    println!("  {} al", instruction);
    println!("  movzb rax, al");
    println!("  push rax");
}

fn print_assembly_arith_expr(expr: Box<ArithExpr>, meta_info: &mut MetaInfo) {
    match *expr {
        ArithExpr::Factor(factor) => print_assembly_factor(factor, meta_info),
        ArithExpr::Add(expr, factor) => {
            print_assembly_arith_expr(expr, meta_info);
            print_assembly_factor(factor, meta_info);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  add rax, rdi");
            println!("  push rax");
        }
        ArithExpr::Sub(expr, factor) => {
            print_assembly_arith_expr(expr, meta_info);
            print_assembly_factor(factor, meta_info);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  sub rax, rdi");
            println!("  push rax");
        }
    }
}

fn print_assembly_factor(factor: Box<Factor>, meta_info: &mut MetaInfo) {
    match *factor {
        Factor::Unary(unary) => print_assembly_unary(unary, meta_info),
        Factor::Mul(factor, unary) => {
            print_assembly_factor(factor, meta_info);
            print_assembly_unary(unary, meta_info);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mul rdi");
            println!("  push rax");
        }
        Factor::Div(factor, unary) => {
            print_assembly_factor(factor, meta_info);
            print_assembly_unary(unary, meta_info);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  cqo");
            println!("  idiv rdi");
            println!("  push rax");
        }
    }
}

fn print_assembly_unary(unary: Box<Unary>, meta_info: &mut MetaInfo) {
    match *unary {
        Unary::Atom(atom) => print_assembly_atom(atom, meta_info),
        Unary::Neg(atom) => {
            print_assembly_atom(atom, meta_info);
            println!("  pop rax");
            println!("  neg rax");
            println!("  push rax");
        }
    }
}

fn print_assembly_atom(atom: Box<Atom>, meta_info: &mut MetaInfo) {
    match *atom {
        Atom::Number(n) => println!("  push {}", n),
        Atom::Expr(expr) => print_assembly_expr(expr, meta_info),
        Atom::Variable(lval) => {
            let id = get_variable_id_and_register_it(&lval, meta_info);
            println!("  mov rax, rbp");
            println!("  sub rax, {}", (id + 1) * 8);
            println!("  push [rax]");
        }
    }
}
