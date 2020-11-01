use std::{error::Error, fmt};

use crate::tokens::Token;
#[derive(Debug, Clone)]
pub struct LoxError {
    message: String,
}

impl LoxError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: String::from(msg),
        }
    }
}

impl Error for LoxError {}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Clone)]
pub struct ParserError {
    token: Option<Token>,
}

impl ParserError {
    pub fn new(token: Option<Token>) -> Self {
        Self { token }
    }
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token {
            Some(t) => format!(
                "Parsing error: Unexpected token '{}'({:?}) at line {}",
                t.lexeme, t.variant, t.line
            ),
            None => "Parsing error: Unexpectedly reached end of file".to_string(),
        };
        Ok(())
    }
}
