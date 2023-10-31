use std::env;

#[derive(Debug)]
enum CompilerError {
    InvalidNumberOfArguments,
    InvalidNumberFormat,
}

fn main() -> Result<(), CompilerError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("The number of arguments is invalid");
        return Err(CompilerError::InvalidNumberOfArguments);
    }

    let num = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("The number format is invalid");
            return Err(CompilerError::InvalidNumberFormat);
        }
    };

    println!(".intel_syntax noprefix");
    println!(".global main\n");
    println!("main:");
    println!("  mov rax, {:?}", num);
    println!("  ret");
    Ok(())
}
