#[derive(Debug)]
pub enum UnaryOperator {
    Bang,
    Minus,
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

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Expr {
    pub fn binary(operator: BinaryOperator, left: Expr, right: Expr) -> Self {
        Expr::Binary(Binary {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        })
    }

    pub fn logical(operator: LogicalOperator, left: Expr, right: Expr) -> Self {
        Expr::Logical(Logical {
            left: Box::new(left),
            right: Box::new(right),
            operator,
        })
    }

    pub fn unary(operator: UnaryOperator, unary: Expr) -> Self {
        Expr::Unary(Unary {
            operator,
            unary: Box::new(unary),
        })
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
