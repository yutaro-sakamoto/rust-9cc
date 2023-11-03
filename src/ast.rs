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
    ArithExpr(Box<ArithExpr>),
}

pub fn print_assembly(expr: Box<ArithExpr>) {
    println!(".intel_syntax noprefix");
    println!(".global main\n");
    println!("main:");
    print_assembly_arith_expr(expr);
    println!("  pop rax");
    println!("  ret");
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
        Atom::ArithExpr(expr) => print_assembly_arith_expr(expr),
    }
}
