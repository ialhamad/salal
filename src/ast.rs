use crate::tokens::Token;

#[derive(Debug)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
#[derive(Debug)]
pub struct Grouping {
    pub expr: Box<Expr>,
}
#[derive(Debug)]
pub enum Literal {
    NilLiteral,
    BoolLiteral(bool),
    StringLiteral(String),
    NumberLiteral(f64),
}
#[derive(Debug)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}
#[derive(Debug)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Logical(Logical),
}

impl Expr {
    pub fn binary(operator: Token, left: Expr, right: Expr) -> Self {
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

    pub fn grouping(grouping: Expr) -> Self {
        Expr::Grouping(Grouping {
            expr: Box::new(grouping),
        })
    }
    pub fn unary(operator: Token, unary: Expr) -> Self {
        Expr::Unary(Unary {
            operator,
            right: Box::new(unary),
        })
    }
}
