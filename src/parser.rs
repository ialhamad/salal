use crate::{
    ast::{Expr, Literal},
    errors::ParserError,
    tokens::{Token, TokenVariant::*},
};
use std::result::Result as StdResult;

type Result<T> = StdResult<T, ParserError>;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
}

impl Parser {
    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }
    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        loop {
            match self.current() {
                Some(token) if token.variant == BangEqual || token.variant == EqualEqual => {
                    self.advance();
                    let right = self.comparison()?;
                    expr = Expr::binary(token, expr, right);
                }
                Some(_) => break,
                None => break,
            }
        }
        Ok(expr)
    }
    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        loop {
            match self.current() {
                Some(token)
                    if token.variant == Greater
                        || token.variant == GreaterEqual
                        || token.variant == Less
                        || token.variant == LessEqual =>
                {
                    self.advance();
                    let right = self.term()?;
                    expr = Expr::binary(token, expr, right);
                }
                Some(_) => break,
                None => break,
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        loop {
            match self.current() {
                Some(token) if token.variant == Plus || token.variant == Minus => {
                    self.advance();
                    let right = self.factor()?;
                    expr = Expr::binary(token, expr, right);
                }
                Some(_) => break,
                None => return Err(ParserError::new(None)),
            }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        loop {
            match self.current() {
                Some(token) if token.variant == Slash || token.variant == Star => {
                    self.advance();
                    let right = self.unary()?;
                    expr = Expr::binary(token, expr, right);
                }
                Some(_) => break,
                None => return Err(ParserError::new(None)),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        match self.current() {
            Some(token) if token.variant == Slash || token.variant == Minus => {
                self.advance();
                let right = self.unary()?;
                Ok(Expr::unary(token, right))
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr> {
        match self.current() {
            Some(Token { variant: False, .. }) => {
                self.advance();
                Ok(Expr::Literal(Literal::BoolLiteral(false)))
            }
            Some(Token { variant: True, .. }) => {
                self.advance();
                Ok(Expr::Literal(Literal::BoolLiteral(true)))
            }
            Some(Token { variant: Nil, .. }) => {
                self.advance();
                Ok(Expr::Literal(Literal::NilLiteral))
            }
            Some(Token {
                variant: Number(n), ..
            }) => {
                self.advance();
                Ok(Expr::Literal(Literal::NumberLiteral(n)))
            }
            Some(Token {
                variant: String(s), ..
            }) => {
                self.advance();
                Ok(Expr::Literal(Literal::StringLiteral(s)))
            }
            Some(Token {
                variant: LeftParen, ..
            }) => {
                self.advance();
                let expr = self.expression()?;
                match self.current() {
                    Some(Token {
                        variant: RightParen,
                        ..
                    }) => {
                        self.advance();
                        Ok(Expr::grouping(expr))
                    }
                    unexpected => Err(ParserError::new(unexpected)),
                }
            }
            unexpected @ Some(_) => Err(ParserError::new(unexpected)),
            None => Err(ParserError::new(None)),
        }
    }

    fn current(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.variant == Eof,
            _ => false,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<Token> {
        self.tokens.get(self.current - 1).cloned()
    }
}
