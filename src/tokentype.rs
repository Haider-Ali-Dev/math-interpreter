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
    LeftParen,
    Modulo,
    True,
    False
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

pub enum Type {
    Number(isize),
    String(String),
    Boolean(bool)
}

#[derive(Debug, Clone, PartialEq, Eq,Deserialize, Serialize)]

pub struct Token {
    pub tty: TokenType,
    pub value: Option<Type>,
    pub line: usize
}