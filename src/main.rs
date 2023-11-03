use std::env;

#[derive(Debug)]
enum CompilerError {
    InvalidNumberOfArguments,
    InvalidExpression,
}

fn main() -> Result<(), CompilerError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("The number of arguments is invalid");
        return Err(CompilerError::InvalidNumberOfArguments);
    }

    println!(".intel_syntax noprefix");
    println!(".global main\n");
    println!("main:");

    let expr_bytes = args[1].as_bytes();
    let mut i = 0;
    let mut num = 0;
    while i < expr_bytes.len() && expr_bytes[i].is_ascii_digit() {
        num = num * 10 + (expr_bytes[i] - '0' as u8) as i32;
        i += 1;
    }
    println!("  mov rax, {:?}", num);
    num = 0;
    let mut prev_operator_is_plus = true;
    let mut first_operator = true;
    while i < expr_bytes.len() {
        let c = expr_bytes[i];
        if c.is_ascii_digit() {
            num = num * 10 + (c - '0' as u8) as i32;
            i += 1;
            continue;
        } else if c == ('+' as u8) || c == ('-' as u8) {
            if first_operator {
                first_operator = false;
            } else if prev_operator_is_plus {
                println!("  add rax, {:?}", num);
            } else {
                println!("  sub rax, {:?}", num);
            }
            num = 0;
            prev_operator_is_plus = c == ('+' as u8);
            i += 1;
            continue;
        } else {
            eprintln!("The number format is invalid");
            return Err(CompilerError::InvalidExpression);
        }
    }
    if !first_operator && prev_operator_is_plus {
        println!("  add rax, {:?}", num);
    } else {
        println!("  sub rax, {:?}", num);
    }

    println!("  ret");
    Ok(())
}
