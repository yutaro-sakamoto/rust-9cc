use std::collections::HashMap;
use std::vec::Vec;

use crate::assembly::*;

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
    let mut assembly: Assembly = Vec::new();
    for statement in program.statements {
        assembly.append(&mut get_assembly_statement(Box::new(statement), meta_info));
        assembly.push(Instruction::Pop(Operand::Register(Register::RAX)));
    }
    print_assembly_code(&assembly);
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

pub fn get_assembly_statement(statement: Box<Statement>, meta_info: &mut MetaInfo) -> Assembly {
    match *statement {
        Statement::Expr(expr) => get_assembly_expr(expr, meta_info),
        Statement::Assign(left, expr) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_lval(left, meta_info));
            assembly.append(&mut get_assembly_expr(expr, meta_info));
            assembly.push(Instruction::Pop(Operand::Register(Register::RDI)));
            assembly.push(Instruction::Pop(Operand::Register(Register::RAX)));
            assembly.push(Instruction::Mov(
                Operand::Memory(Register::RAX),
                Operand::Register(Register::RDI),
            ));
            assembly.push(Instruction::Push(Operand::Register(Register::RDI)));
            assembly
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

fn get_assembly_lval(lval: String, meta_info: &mut MetaInfo) -> Assembly {
    let id = get_variable_id_and_register_it(&lval, meta_info);
    let mut assembly: Assembly = Vec::new();
    assembly.push(Instruction::Mov(
        Operand::Register(Register::RAX),
        Operand::Register(Register::RBP),
    ));
    assembly.push(Instruction::Sub(
        Operand::Register(Register::RAX),
        Operand::Immediate((id + 1) as i32 * 8),
    ));
    assembly.push(Instruction::Push(Operand::Register(Register::RAX)));
    assembly
}

fn get_assembly_expr(expr: Box<Expr>, meta_info: &mut MetaInfo) -> Assembly {
    match *expr {
        Expr::ArithExpr(arith_expr) => get_assembly_arith_expr(arith_expr, meta_info),
        //Expr::Equal(left, right) => get_compare_instruction("sete", left, right, meta_info),
        Expr::Equal(left, right) => {
            get_compare_instruction(&|o: Operand| Instruction::Sete(o), left, right, meta_info)
        }
        Expr::NotEqual(left, right) => {
            get_compare_instruction(&|o: Operand| Instruction::Setne(o), left, right, meta_info)
        }
        Expr::Less(left, right) => {
            get_compare_instruction(&|o: Operand| Instruction::Setl(o), left, right, meta_info)
        }
        Expr::LessOrEqual(left, right) => {
            get_compare_instruction(&|o: Operand| Instruction::Setle(o), left, right, meta_info)
        }
    }
}

fn get_compare_instruction(
    instruction: &dyn Fn(Operand) -> Instruction,
    left: Box<ArithExpr>,
    right: Box<ArithExpr>,
    meta_info: &mut MetaInfo,
) -> Assembly {
    let mut assembly: Assembly = Vec::new();
    assembly.append(&mut get_assembly_arith_expr(left, meta_info));
    assembly.append(&mut get_assembly_arith_expr(right, meta_info));
    assembly.push(Instruction::Pop(Operand::Register(Register::RDI)));
    assembly.push(Instruction::Pop(Operand::Register(Register::RAX)));
    assembly.push(Instruction::Cmp(
        Operand::Register(Register::RAX),
        Operand::Register(Register::RDI),
    ));
    assembly.push(instruction(Operand::Register(Register::AL)));
    assembly.push(Instruction::Movzb(
        Operand::Register(Register::RAX),
        Operand::Register(Register::AL),
    ));
    assembly.push(Instruction::Push(Operand::Register(Register::RAX)));
    assembly
}

fn get_assembly_arith_expr(expr: Box<ArithExpr>, meta_info: &mut MetaInfo) -> Assembly {
    match *expr {
        ArithExpr::Factor(factor) => get_assembly_factor(factor, meta_info),
        ArithExpr::Add(expr, factor) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_arith_expr(expr, meta_info));
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.push(Instruction::Pop(Operand::Register(Register::RDI)));
            assembly.push(Instruction::Pop(Operand::Register(Register::RAX)));
            assembly.push(Instruction::Add(
                Operand::Register(Register::RAX),
                Operand::Register(Register::RDI),
            ));
            assembly.push(Instruction::Push(Operand::Register(Register::RAX)));
            assembly
        }
        ArithExpr::Sub(expr, factor) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_arith_expr(expr, meta_info));
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.push(Instruction::Pop(Operand::Register(Register::RDI)));
            assembly.push(Instruction::Pop(Operand::Register(Register::RAX)));
            assembly.push(Instruction::Sub(
                Operand::Register(Register::RAX),
                Operand::Register(Register::RDI),
            ));
            assembly.push(Instruction::Push(Operand::Register(Register::RAX)));
            assembly
        }
    }
}

fn get_assembly_factor(factor: Box<Factor>, meta_info: &mut MetaInfo) -> Assembly {
    match *factor {
        Factor::Unary(unary) => get_assembly_unary(unary, meta_info),
        Factor::Mul(factor, unary) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut get_assembly_unary(unary, meta_info));
            assembly.push(Instruction::Pop(Operand::Register(Register::RDI)));
            assembly.push(Instruction::Pop(Operand::Register(Register::RAX)));
            assembly.push(Instruction::Mul(Operand::Register(Register::RDI)));
            assembly.push(Instruction::Push(Operand::Register(Register::RAX)));
            assembly
        }
        Factor::Div(factor, unary) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut get_assembly_unary(unary, meta_info));
            assembly.push(Instruction::Pop(Operand::Register(Register::RDI)));
            assembly.push(Instruction::Pop(Operand::Register(Register::RAX)));
            assembly.push(Instruction::Cqo);
            assembly.push(Instruction::Idiv(Operand::Register(Register::RDI)));
            assembly.push(Instruction::Push(Operand::Register(Register::RAX)));
            assembly
        }
    }
}

fn get_assembly_unary(unary: Box<Unary>, meta_info: &mut MetaInfo) -> Assembly {
    match *unary {
        Unary::Atom(atom) => get_assembly_atom(atom, meta_info),
        Unary::Neg(atom) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_atom(atom, meta_info));
            assembly.push(Instruction::Pop(Operand::Register(Register::RAX)));
            assembly.push(Instruction::Neg(Operand::Register(Register::RAX)));
            assembly.push(Instruction::Push(Operand::Register(Register::RAX)));
            assembly
        }
    }
}

fn get_assembly_atom(atom: Box<Atom>, meta_info: &mut MetaInfo) -> Assembly {
    match *atom {
        Atom::Number(n) => vec![Instruction::Push(Operand::Immediate(n))],
        Atom::Expr(expr) => get_assembly_expr(expr, meta_info),
        Atom::Variable(lval) => {
            let mut assembly: Assembly = Vec::new();
            let id = get_variable_id_and_register_it(&lval, meta_info);
            assembly.push(Instruction::Mov(
                Operand::Register(Register::RAX),
                Operand::Register(Register::RBP),
            ));
            assembly.push(Instruction::Sub(
                Operand::Register(Register::RAX),
                Operand::Immediate((id + 1) as i32 * 8),
            ));
            assembly.push(Instruction::Push(Operand::Memory(Register::RAX)));
            assembly
        }
    }
}
