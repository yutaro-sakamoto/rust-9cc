#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(
    #[allow(clippy::all)]
    #[allow(unused)]
    pub parser
);
pub mod assembly;
pub mod ast;
pub mod compile_error;
pub mod gen_code;
use crate::gen_code::print_assembly;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("The number of arguments is invalid");
        return ExitCode::from(1);
    }
    match parser::ProgramParser::new().parse(&args[1]) {
        Ok(parse_tree) => {
            if let Err(e) = print_assembly(&parse_tree) {
                eprintln!("Failed to compile: {}", e);
                return ExitCode::from(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to parse: {}", e);
            return ExitCode::from(1);
        }
    };

    ExitCode::from(0)
}
