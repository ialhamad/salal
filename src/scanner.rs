use crate::{
    errors::SalalError,
    tokens::{Token, TokenType},
};
use anyhow::Result;
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and"    => TokenType::And,
    "class"  => TokenType::Class,
    "else"   => TokenType::Else,
    "false"  => TokenType::False,
    "for"    => TokenType::Fun,
    "fun"    => TokenType::For,
    "if"     => TokenType::If,
    "nil"    => TokenType::Nil,
    "or"     => TokenType::Or,
    "print"  => TokenType::Print,
    "return" => TokenType::Return,
    "super"  => TokenType::Super,
    "this"   => TokenType::This,
    "true"   => TokenType::True,
    "var"    => TokenType::Var,
    "while"  => TokenType::While,
};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = source.chars().into_iter().collect::<Vec<_>>();
        Self {
            source: chars,
            current: 0,
            start: 0,
            line: 1,
        }
    }
    pub fn scan(&mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_tokens()?;
            if let Some(token) = token {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(TokenType::Eof, String::new(), self.line));
        Ok(tokens)
    }
    pub fn scan_tokens(&mut self) -> Result<Option<Token>, SalalError> {
        self.skip_whitespace();
        let c = self.advance();
        match c {
            '(' => Ok(self.make_token(TokenType::LeftParen)),
            ')' => Ok(self.make_token(TokenType::RightParen)),
            '{' => Ok(self.make_token(TokenType::LeftBrace)),
            '}' => Ok(self.make_token(TokenType::RightBrace)),
            ',' => Ok(self.make_token(TokenType::Comma)),
            '.' => Ok(self.make_token(TokenType::Dot)),
            '-' => Ok(self.make_token(TokenType::Minus)),
            '+' => Ok(self.make_token(TokenType::Plus)),
            ';' => Ok(self.make_token(TokenType::Semicolon)),
            '*' => Ok(self.make_token(TokenType::Star)),
            '!' => Ok(if self.match_char('=') {
                self.make_token(TokenType::BangEqual)
            } else {
                self.make_token(TokenType::Bang)
            }),
            '=' => Ok(if self.match_char('=') {
                self.make_token(TokenType::EqualEqual)
            } else {
                self.make_token(TokenType::Equal)
            }),
            '<' => Ok(if self.match_char('=') {
                self.make_token(TokenType::LessEqual)
            } else {
                self.make_token(TokenType::Less)
            }),
            '>' => Ok(if self.match_char('=') {
                self.make_token(TokenType::GreaterEqual)
            } else {
                self.make_token(TokenType::Greater)
            }),
            '"' => self.string(),
            _ if c.is_ascii_digit() => self.number(),
            _ if c.is_ascii_alphanumeric() => self.identifier(),
            _ => Err(SalalError::new("Unexpected character.")),
        }
    }
    fn is_at_end(&self) -> bool {
        self.source.is_empty() || self.current >= self.source.len()
    }
    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }
    pub fn make_token(&self, ty: TokenType) -> Option<Token> {
        let lexeme = self.source[self.start..self.current].into_iter().collect();
        Some(Token::new(ty, lexeme, self.line))
    }
    pub fn match_char(&mut self, c: char) -> bool {
        if !self.is_at_end() && self.source[self.current] == c {
            self.current += 1;
            return true;
        }
        false
    }
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current].into()
        }
    }
    pub fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }
    pub fn string(&mut self) -> Result<Option<Token>, SalalError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }
        if self.is_at_end() {
            return Err(SalalError::new("Unterminated string."));
        }
        self.current += 1;

        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        Ok(Some(Token::new(TokenType::String, value, self.line)))
    }
    pub fn number(&mut self) -> Result<Option<Token>, SalalError> {
        while self.peek().is_numeric() {
            self.current += 1;
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.current += 1;
            while self.peek().is_ascii_digit() {
                self.current += 1;
            }
        }
        let value = self.source[self.start..self.current].into_iter().collect();
        Ok(Some(Token::new(TokenType::Number, value, self.line)))
    }
    pub fn identifier(&mut self) -> Result<Option<Token>, SalalError> {
        while self.peek().is_ascii_alphanumeric() {
            self.current += 1;
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        if let Some(keyword) = KEYWORDS.get(value.as_str()) {
            Ok(Some(Token::new(keyword.clone(), value, self.line)))
        } else {
            Ok(Some(Token::new(TokenType::Identifier, value, self.line)))
        }
    }
    pub fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\t' | '\r' => {
                    self.current += 1;
                }
                '\n' => {
                    self.line += 1;
                    self.current += 1;
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.current += 1;
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
}
