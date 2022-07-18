use core::panic;

use crate::{parser::Expr, tokentype::TokenType, tokentype::Type};

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(e: &Expr) -> Result<f64, String> {
        match e {
            Expr::BinaryExpr { right, op, left } => match op.tty {
                TokenType::Divide => {
                    Ok(Interpreter::interpret(left)? / Interpreter::interpret(right)?)
                }
                TokenType::Minus => {
                    Ok(Interpreter::interpret(left)? - Interpreter::interpret(right)?)
                }
                TokenType::Plus => {
                    Ok(Interpreter::interpret(left)? + Interpreter::interpret(right)?)
                }
                TokenType::Star => {
                    Ok(Interpreter::interpret(left)? * Interpreter::interpret(right)?)
                }
                _ => Err("Unknown Operation".to_owned()),
            },
            Expr::LiteralExpr { value } => match value.clone().unwrap() {
                Type::Number(a) => Ok(a as f64),
                Type::String(a) => match a.as_str() {
                    "pi" => Ok(3.141592653589793238),
                    _ => Err("Wrong math variable".to_string()),
                },
            },
            Expr::UnaryExpr { op, right } => match op.tty {
                TokenType::Minus => Ok(-Interpreter::interpret(&right)?),
                _ => Err("Unknow Operation".to_owned()),
            },
        }
    }
}
