use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub enum TokenType {
    Divide,
    Plus,
    Minus,
    Star,
    Number,
    Var,
    RightParen,
    LeftParen
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub enum Type {
    Number(isize),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq,Deserialize, Serialize)]

pub struct Token {
    pub tty: TokenType,
    pub value: Option<Type>,
    pub line: usize
}