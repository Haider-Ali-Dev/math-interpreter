use crate::tokentype::{Token, TokenType, Type};
use substring::*;
pub struct Lexer {
    pub code: String,
    line: usize,
    current: usize,
    broken_code: Vec<char>,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
            line: 1,
            current: 0,
            broken_code: code.chars().collect::<Vec<_>>(),
            tokens: vec![],
        }
    }
    pub fn lex(&mut self) -> Vec<Token> {
        // let mut tokens = vec![];
        while !self.is_at_end() {
            let c = self.current_char().unwrap();
            match c {
                '+' => self.tokens.push(Token {
                    tty: TokenType::Plus,
                    value: None,
                    line: self.line,
                }),
                '-' => self.tokens.push(Token {
                    tty: TokenType::Minus,
                    value: None,
                    line: self.line,
                }),
                '/' => self.tokens.push(Token {
                    tty: TokenType::Divide,
                    value: None,
                    line: self.line,
                }),
                '*' => self.tokens.push(Token {
                    tty: TokenType::Star,
                    value: None,
                    line: self.line,
                }),
                '(' => self.tokens.push(Token {
                    tty: TokenType::RightParen,
                    value: None,
                    line: self.line,
                }),
                ')' => self.tokens.push(Token {
                    tty: TokenType::LeftParen,
                    value: None,
                    line: self.line,
                }),
                a => {
                    if a.is_ascii_digit() {
                        let val = a.to_digit(10).unwrap();
                        self.tokens.push(Token {
                            value: Some(Type::Number(val as isize)),
                            tty: TokenType::Number,
                            line: self.line,
                        })
                    } else if a.clone() == 'v' {
                        let mut v_index = vec![];
                        v_index.push(self.current_char().unwrap().clone());
                        if self.next() == Some(&'a') {
                            v_index.push(self.current_char().unwrap().clone());
                        }
                        if self.next() == Some(&'r') {
                            v_index.push(self.current_char().unwrap().clone());
                        }

                        let v_as_string = v_index.iter().collect::<String>();
                        if v_as_string.as_str() == "var" {
                            let mut value = String::new();
                            while self.next() != Some(&')') {
                                println!("{}", self.current_char().unwrap());
                                if self.current_char() == Some(&'(') {
                                    continue;
                                }

                                value.push(*self.current_char().unwrap());
                            }
                            self.tokens.push(Token {
                                tty: TokenType::Var,
                                value: Some(Type::String(value)),
                                line: self.line,
                            });
                        }
                    }
                }
            };
            self.next();
        }
        self.tokens.clone()
    }
    pub fn var_value(&self) -> String {
        let value = self.code.substring(1, self.current - 1);
        value.to_owned()
    }
    pub fn is_at_end(&self) -> bool {
        self.code.len() == self.current
    }
    pub fn next(&mut self) -> Option<&char> {
        self.current += 1;
        self.broken_code.get(self.current)
    }
    pub fn peek_next(&self, i: usize) -> Option<&char> {
        self.broken_code.get(self.current + i)
    }
    pub fn current_char(&self) -> Option<&char> {
        self.broken_code.get(self.current)
    }

    
}
