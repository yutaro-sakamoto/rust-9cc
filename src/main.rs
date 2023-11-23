#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub parser
);
pub mod assembly;
pub mod ast;
pub mod gen_code;
use crate::gen_code::print_assembly;
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
    match parser::ProgramParser::new().parse(&args[1]) {
        Ok(parse_tree) => {
            print_assembly(&parse_tree);
        }
        Err(e) => {
            eprintln!("Failed to parse: {}", e);
            return Err(CompilerError::InvalidExpression);
        }
    };

    Ok(())
}
