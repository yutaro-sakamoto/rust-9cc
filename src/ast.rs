use std::vec::Vec;
pub struct Program {
    pub statements: Vec<Statement>,
}
pub enum Statement {
    Expr(Box<Expr>),
    Assign(u32, Box<Expr>),
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
    Variable(u32),
}

pub fn print_assembly(program: Box<Program>) {
    let number_of_variables = 1;
    println!(".intel_syntax noprefix");
    println!(".global main\n");
    println!("main:");
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, {}", 8 * number_of_variables);
    for statement in program.statements {
        print_assembly_statement(Box::new(statement));
        println!("  pop rax");
    }
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

pub fn print_assembly_statement(statement: Box<Statement>) {
    match *statement {
        Statement::Expr(expr) => print_assembly_expr(expr),
        Statement::Assign(left, expr) => {
            print_assembly_lval(left);
            print_assembly_expr(expr);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mov [rax], rdi");
            println!("  push rdi");
        }
    }
}

fn print_assembly_lval(lval: u32) {
    println!("  mov rax, rbp");
    println!("  sub rax, {}", (lval + 1) * 8);
    println!("  push rax");
}

fn print_assembly_expr(expr: Box<Expr>) {
    match *expr {
        Expr::ArithExpr(arith_expr) => print_assembly_arith_expr(arith_expr),
        Expr::Equal(left, right) => print_compare_instruction("sete", left, right),
        Expr::NotEqual(left, right) => print_compare_instruction("setne", left, right),
        Expr::Less(left, right) => print_compare_instruction("setl", left, right),
        Expr::LessOrEqual(left, right) => print_compare_instruction("setle", left, right),
    }
}

fn print_compare_instruction(instruction: &str, left: Box<ArithExpr>, right: Box<ArithExpr>) {
    print_assembly_arith_expr(left);
    print_assembly_arith_expr(right);
    println!("  pop rdi");
    println!("  pop rax");
    println!("  cmp rax, rdi");
    println!("  {} al", instruction);
    println!("  movzb rax, al");
    println!("  push rax");
}

fn print_assembly_arith_expr(expr: Box<ArithExpr>) {
    match *expr {
        ArithExpr::Factor(factor) => print_assembly_factor(factor),
        ArithExpr::Add(expr, factor) => {
            print_assembly_arith_expr(expr);
            print_assembly_factor(factor);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  add rax, rdi");
            println!("  push rax");
        }
        ArithExpr::Sub(expr, factor) => {
            print_assembly_arith_expr(expr);
            print_assembly_factor(factor);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  sub rax, rdi");
            println!("  push rax");
        }
    }
}

fn print_assembly_factor(factor: Box<Factor>) {
    match *factor {
        Factor::Unary(unary) => print_assembly_unary(unary),
        Factor::Mul(factor, unary) => {
            print_assembly_factor(factor);
            print_assembly_unary(unary);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  mul rdi");
            println!("  push rax");
        }
        Factor::Div(factor, unary) => {
            print_assembly_factor(factor);
            print_assembly_unary(unary);
            println!("  pop rdi");
            println!("  pop rax");
            println!("  cqo");
            println!("  idiv rdi");
            println!("  push rax");
        }
    }
}

fn print_assembly_unary(unary: Box<Unary>) {
    match *unary {
        Unary::Atom(atom) => print_assembly_atom(atom),
        Unary::Neg(atom) => {
            print_assembly_atom(atom);
            println!("  pop rax");
            println!("  neg rax");
            println!("  push rax");
        }
    }
}

fn print_assembly_atom(atom: Box<Atom>) {
    match *atom {
        Atom::Number(n) => println!("  push {}", n),
        Atom::Expr(expr) => print_assembly_expr(expr),
        Atom::Variable(lval) => {
            println!("  mov rax, rbp");
            println!("  sub rax, {}", (lval + 1) * 8);
            println!("  push [rax]");
        }
    }
}
