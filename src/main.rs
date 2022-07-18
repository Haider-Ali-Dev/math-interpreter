use lexer::Lexer;
use serde_json::json;
use std::fs::File;
pub mod interpreter;
use std::io::Write;

use crate::interpreter::Interpreter;
use crate::parser::Parser;

pub mod lexer;
pub mod parser;
pub mod tokentype;
fn main() {
    let code ="1 + 1 + var(pi)";
    let mut lexer = Lexer::new(code);
    let e = lexer.lex();
    let ast = &Parser::new(e.clone()).parse();
    match Interpreter::interpret(ast) {
        Ok(e) => {
            println!("{e}")
        },
        Err(e) => {
            eprintln!("{e}")
        }
    }
    
}
