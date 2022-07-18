use lexer::Lexer;
use serde_json::json;
use std::fs::File;
pub mod interpreter;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;

use crate::interpreter::Interpreter;
use crate::parser::Parser;

pub mod lexer;
pub mod parser;
pub mod tokentype;
fn main() {
    let mut buffer = String::new();
    let file = &std::env::args().collect::<Vec<_>>()[1];
    let file_data = File::open(file).unwrap();
    let mut reader = BufReader::new(file_data);
    reader.read_to_string(&mut buffer).unwrap();
    let mut lexer = Lexer::new(&buffer);
    let e = lexer.lex();
    if e.len() != 0 {
        let ast = &Parser::new(e.clone()).parse();
        match Interpreter::interpret(ast) {
            Ok(e) => {
                println!("{e}")
            }
            Err(e) => {
                eprintln!("{e}")
            }
        }
    }
}
