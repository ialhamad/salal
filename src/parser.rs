use crate::{
    ast::Expr,
    tokens::{Token, TokenVariant},
};
use anyhow::Result;

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
    fn equality(&mut self) -> Result<Expr> {
        // Expr expr = comparison();

        // while (match(BANG_EQUAL, EQUAL_EQUAL)) {
        //   Token operator = previous();
        //   Expr right = comparison();
        //   expr = new Expr.Binary(expr, operator, right);
        // }

        // return expr;
        let mut expr = self.comparison();
        while self.match_variants(vec![TokenVariant::BangEqual, TokenVariant::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::binary(operator, expr, right);
        }
        todo!()
    }
    fn comparison(&self) {
        todo!()
    }

    fn match_variants(&self, token_variants: Vec<TokenVariant>) -> bool {
        for variant in token_variants {
            if self.check(variant) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn check(&self, variant: TokenVariant) -> bool {
        if self.is_at_end() {
            return false;
        }
        match self.peek() {
            Some(token) => token.variant == variant,
            _ => false,
        }
    }
    fn is_at_end(&self) -> bool {
        match self.peek() {
            Some(token) => token.variant == TokenVariant::Eof,
            _ => false,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }
}
