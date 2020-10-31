use std::fmt;

use crate::tokens::Token;

#[derive(Debug)]
pub enum UnaryOperator {
    Bang,
    Minus,
}
impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Minus,
    Plus,
    Slash,
    Star,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}
impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: BinaryOperator,
    pub right: Box<Expr>,
}
pub struct Grouping {
    pub expr: Box<Expr>,
}
pub enum Literal {
    NilLiteral,
    BoolLiteral(bool),
    StringLiteral(String),
    NumberLiteral(f64),
}
pub struct Unary {
    pub operator: UnaryOperator,
    pub right: Box<Expr>,
}
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Logical(Logical),
}

impl Expr {
    pub fn binary(operator: BinaryOperator, left: Expr, right: Expr) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        })
    }

    pub fn logical(operator: Token, left: Expr, right: Expr) -> Self {
        Expr::Logical(Logical {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        })
    }

    pub fn unary(operator: UnaryOperator, unary: Expr) -> Self {
        Expr::Unary(Unary {
            operator,
            right: Box::new(unary),
        })
    }
}
