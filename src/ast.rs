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

impl Default for MetaInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub fn print_assembly(program: &Program, meta_info: &mut MetaInfo) {
    println!(".intel_syntax noprefix");
    println!(".global main\n");
    println!("main:");
    let header_code = vec![push(rbp()), mov(rbp(), rsp())];
    let mut main_code: Assembly = Vec::new();
    for statement in program.statements.iter() {
        main_code.append(&mut get_assembly_statement(statement, meta_info));
        main_code.push(pop(rax()));
    }
    let number_of_variables = meta_info.variable_map.len() as i32;
    let sub_rsp_code = sub(rsp(), immediate(8 * number_of_variables));
    let footer_code = vec![mov(rsp(), rbp()), pop(rbp()), ret()];

    print_assembly_code(&header_code);
    print_single_instruction(&sub_rsp_code);
    print_assembly_code(&main_code);
    print_assembly_code(&footer_code);
}

pub fn get_assembly_statement(statement: &Statement, meta_info: &mut MetaInfo) -> Assembly {
    match statement {
        Statement::Expr(expr) => get_assembly_expr(expr, meta_info),
        Statement::Assign(left, expr) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_lval(left, meta_info));
            assembly.append(&mut get_assembly_expr(expr, meta_info));
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                mov(m_rax(), rdi()),
                push(rdi()),
            ]);
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

fn get_assembly_lval(lval: &String, meta_info: &mut MetaInfo) -> Assembly {
    let id = get_variable_id_and_register_it(lval, meta_info);
    vec![
        mov(rax(), rbp()),
        sub(rax(), immediate((id + 1) as i32 * 8)),
        push(rax()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_variable_id_and_register_it() {
        let mut meta_info = MetaInfo::new();
        let a_id = get_variable_id_and_register_it(&"a".to_string(), &mut meta_info);
        let b_id = get_variable_id_and_register_it(&"b".to_string(), &mut meta_info);
        assert_eq!(
            get_variable_id_and_register_it(&"a".to_string(), &mut meta_info),
            a_id
        );
        assert_eq!(
            get_variable_id_and_register_it(&"b".to_string(), &mut meta_info),
            b_id
        );
    }
}

fn get_assembly_expr(expr: &Expr, meta_info: &mut MetaInfo) -> Assembly {
    match expr {
        Expr::ArithExpr(arith_expr) => get_assembly_arith_expr(arith_expr, meta_info),
        Expr::Equal(left, right) => {
            get_compare_instruction(&|o: Operand| sete(o), left, right, meta_info)
        }
        Expr::NotEqual(left, right) => {
            get_compare_instruction(&|o: Operand| setne(o), left, right, meta_info)
        }
        Expr::Less(left, right) => {
            get_compare_instruction(&|o: Operand| setl(o), left, right, meta_info)
        }
        Expr::LessOrEqual(left, right) => {
            get_compare_instruction(&|o: Operand| setle(o), left, right, meta_info)
        }
    }
}

fn get_compare_instruction(
    gen_instruction: &dyn Fn(Operand) -> Instruction,
    left: &ArithExpr,
    right: &ArithExpr,
    meta_info: &mut MetaInfo,
) -> Assembly {
    let mut assembly: Assembly = Vec::new();
    assembly.append(&mut get_assembly_arith_expr(left, meta_info));
    assembly.append(&mut get_assembly_arith_expr(right, meta_info));
    assembly.append(&mut vec![
        pop(rdi()),
        pop(rax()),
        cmp(rax(), rdi()),
        gen_instruction(al()),
        movzb(rax(), al()),
        push(rax()),
    ]);
    assembly
}

fn get_assembly_arith_expr(expr: &ArithExpr, meta_info: &mut MetaInfo) -> Assembly {
    match expr {
        ArithExpr::Factor(factor) => get_assembly_factor(factor, meta_info),
        ArithExpr::Add(expr, factor) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_arith_expr(expr, meta_info));
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                add(rax(), rdi()),
                push(rax()),
            ]);
            assembly
        }
        ArithExpr::Sub(expr, factor) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_arith_expr(expr, meta_info));
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                sub(rax(), rdi()),
                push(rax()),
            ]);
            assembly
        }
    }
}

fn get_assembly_factor(factor: &Factor, meta_info: &mut MetaInfo) -> Assembly {
    match factor {
        Factor::Unary(unary) => get_assembly_unary(unary, meta_info),
        Factor::Mul(factor, unary) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut get_assembly_unary(unary, meta_info));
            assembly.append(&mut vec![pop(rdi()), pop(rax()), mul(rdi()), push(rax())]);
            assembly
        }
        Factor::Div(factor, unary) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_factor(factor, meta_info));
            assembly.append(&mut get_assembly_unary(unary, meta_info));
            assembly.append(&mut vec![
                pop(rdi()),
                pop(rax()),
                cqo(),
                idiv(rdi()),
                push(rax()),
            ]);
            assembly
        }
    }
}

fn get_assembly_unary(unary: &Unary, meta_info: &mut MetaInfo) -> Assembly {
    match unary {
        Unary::Atom(atom) => get_assembly_atom(atom, meta_info),
        Unary::Neg(atom) => {
            let mut assembly: Assembly = Vec::new();
            assembly.append(&mut get_assembly_atom(atom, meta_info));
            assembly.append(&mut vec![pop(rax()), neg(rax()), push(rax())]);
            assembly
        }
    }
}

fn get_assembly_atom(atom: &Atom, meta_info: &mut MetaInfo) -> Assembly {
    match atom {
        Atom::Number(n) => vec![Instruction::Push(Operand::Immediate(*n))],
        Atom::Expr(expr) => get_assembly_expr(expr, meta_info),
        Atom::Variable(lval) => {
            let id = get_variable_id_and_register_it(lval, meta_info);
            vec![
                mov(rax(), rbp()),
                sub(rax(), immediate((id + 1) as i32 * 8)),
                push(m_rax()),
            ]
        }
    }
}
