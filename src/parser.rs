use crate::tokentype::{Token, TokenType, Type};
use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Expr {
    BinaryExpr {
        right: Box<Expr>,
        op: Token,
        left: Box<Expr>,
    },
    LiteralExpr {
        value: Option<Type>,
    },
    UnaryExpr {
        op: Token,
        right: Box<Expr>,
    },
}
#[derive(Debug, Clone)]
pub struct Parser {
    current: usize,
    pub lex: Vec<Token>,
}
impl Parser {
    pub fn new(lex: Vec<Token>) -> Self {
        Self { current: 0, lex }
    }
    pub fn parse(&mut self) -> Expr {
        
        self.expression()
    }

    pub fn expression(&mut self) -> Expr {
        self.term()
    }

    pub fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_next(vec![TokenType::Minus, TokenType::Plus]) {
            let op = self.get_previous();
            let right = self.factor();
            expr = Expr::BinaryExpr {
                right: Box::new(right),
                op: op,
                left: Box::new(expr),
            };
        }
        expr
    }

    pub fn factor(&mut self) ->  Expr {
        let mut expr = self.unary();
        while self.match_next(vec![TokenType::Star, TokenType::Divide, TokenType::Modulo]) {
            let op = self.get_previous();
            let right = self.unary();
            expr = Expr::BinaryExpr { right: Box::new(right), op: op, left: Box::new(expr) }
        }
        expr
    }

    pub fn unary(&mut self) -> Expr {
        if self.match_next(vec![TokenType::Minus]) {
            let op = self.current_token();
            let right = self.unary();
            return Expr::UnaryExpr { op: op, right: Box::new(right) }
        }
        self.primary()
    }

    pub fn primary(&mut self) -> Expr {
        if self.match_next(vec![TokenType::Number]) {
            let a = self.get_previous();
            Expr::LiteralExpr { value: a.value }
        } else {
            let a = self.current_token();
            Expr::LiteralExpr { value: a.value }
        }
    }

    pub fn match_next(&mut self, v: Vec<TokenType>) -> bool {
        if self.is_at_end() {
            return false;
        }
        for token in v {
            if token == self.current_token_type() {
                self.next();
                return true;
            }
        }
        false
    }

    pub fn next(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.lex.len() == self.current
    }

    fn get_previous(&self) -> Token {
        self.lex.get(self.current - 1).unwrap().clone()
    }

    fn current_token_type(&self) -> TokenType {
        self.lex
            .get(self.current)
            .unwrap_or_else(|| &Token {
                tty: TokenType::Minus,
                value: None,
                line: 1,
            })
            .clone()
            .tty
    }
    fn current_token(&self) -> Token {
        self.lex
            .get(self.current)
            .unwrap_or_else(|| &Token {
                tty: TokenType::Minus,
                value: None,
                line: 1,
            })
            .clone()
    }

    fn peek_next(&self) -> Token {
        self.lex.get(self.current + 1).unwrap().clone()
    }
}

// pub fn evaluate(expr: Expr) {
//     match expr {
//         Expr::BinaryExpr { right, op, left } => {
//             let operation = match op {

//             }
//         }
//     }
// }
