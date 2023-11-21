#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub parser
);
pub mod assembly;
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
    match parser::ProgramParser::new().parse(&args[1]) {
        Ok(parse_tree) => {
            let mut meta_info = MetaInfo::new();
            print_assembly(&parse_tree, &mut meta_info);
        }
        Err(e) => {
            eprintln!("Failed to parse: {}", e);
            return Err(CompilerError::InvalidExpression);
        }
    };

    Ok(())
}
