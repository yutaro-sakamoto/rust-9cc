#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);
pub mod ast;
use crate::ast::*;
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
    match parser::ExprParser::new().parse(&args[1]) {
        Ok(parse_tree) => print_assembly(Box::new(parse_tree)),
        Err(e) => {
            eprintln!("Failed to parse: {}", e);
            return Err(CompilerError::InvalidExpression);
        }
    };

    Ok(())
}
