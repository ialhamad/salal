use std::fmt;

#[derive(Clone, Debug)]
pub struct Token {
    pub variant: TokenVariant,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(variant: TokenVariant, lexeme: String, line: usize) -> Self {
        Self {
            variant,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {}", self.variant, self.lexeme)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenVariant {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}
