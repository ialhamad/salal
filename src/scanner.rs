use crate::tokens::{Token, TokenVariant};
use anyhow::Result;
use phf::phf_map;

static KEYWORDS: phf::Map<&'static str, TokenVariant> = phf_map! {
    "and"    => TokenVariant::And,
    "class"  => TokenVariant::Class,
    "else"   => TokenVariant::Else,
    "false"  => TokenVariant::False,
    "for"    => TokenVariant::Fun,
    "fun"    => TokenVariant::For,
    "if"     => TokenVariant::If,
    "nil"    => TokenVariant::Nil,
    "or"     => TokenVariant::Or,
    "print"  => TokenVariant::Print,
    "return" => TokenVariant::Return,
    "super"  => TokenVariant::Super,
    "this"   => TokenVariant::This,
    "true"   => TokenVariant::True,
    "var"    => TokenVariant::Var,
    "while"  => TokenVariant::While,
};

pub struct Scanner {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,

    has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let chars = source.chars().into_iter().collect::<Vec<_>>();
        Self {
            source: chars,
            current: 0,
            start: 0,
            line: 1,
            has_error: false,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_tokens();
            if let Some(token) = token {
                tokens.push(token);
            }
        }

        tokens.push(Token::new(TokenVariant::Eof, String::new(), self.line));
        Ok(tokens)
    }

    pub fn scan_tokens(&mut self) -> Option<Token> {
        let c = self.advance();
        match c {
            '(' => self.make_token(TokenVariant::LeftParen),
            ')' => self.make_token(TokenVariant::RightParen),
            '{' => self.make_token(TokenVariant::LeftBrace),
            '}' => self.make_token(TokenVariant::RightBrace),
            ',' => self.make_token(TokenVariant::Comma),
            '.' => self.make_token(TokenVariant::Dot),
            '-' => self.make_token(TokenVariant::Minus),
            '+' => self.make_token(TokenVariant::Plus),
            ';' => self.make_token(TokenVariant::Semicolon),
            '*' => self.make_token(TokenVariant::Star),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenVariant::BangEqual)
                } else {
                    self.make_token(TokenVariant::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenVariant::EqualEqual)
                } else {
                    self.make_token(TokenVariant::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenVariant::LessEqual)
                } else {
                    self.make_token(TokenVariant::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenVariant::GreaterEqual)
                } else {
                    self.make_token(TokenVariant::Greater)
                }
            }
            '"' => self.string(),
            _ if c.is_ascii_digit() => self.number(),
            _ if c.is_ascii_alphanumeric() => self.identifier(),
            '/' if self.peek_next() == '/' => {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.current += 1;
                }
                None
            }
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            _ => {
                self.has_error = true;
                None
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.source.is_empty() || self.current >= self.source.len()
    }

    pub fn advance(&mut self) -> char {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.source[self.current - 1]
    }

    pub fn make_token(&self, variant: TokenVariant) -> Option<Token> {
        let lexeme = self.source[self.start..self.current].iter().collect();
        Some(Token::new(variant, lexeme, self.line))
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
            self.source[self.current]
        }
    }

    pub fn peek_next(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    pub fn string(&mut self) -> Option<Token> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.current += 1;
        }
        if self.is_at_end() {
            return None;
        }
        self.current += 1;

        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect::<String>();
        Some(Token::new(
            TokenVariant::String(value.clone()),
            value,
            self.line,
        ))
    }

    pub fn number(&mut self) -> Option<Token> {
        while self.peek().is_numeric() {
            self.current += 1;
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.current += 1;
            while self.peek().is_ascii_digit() {
                self.current += 1;
            }
        }

        let value = self.source[self.start..self.current]
            .iter()
            .collect::<String>()
            .parse::<f64>();

        match value {
            Ok(num) => Some(Token::new(
                TokenVariant::Number(num),
                num.to_string(),
                self.line,
            )),
            Err(_) => None,
        }
    }

    pub fn identifier(&mut self) -> Option<Token> {
        while self.peek().is_ascii_alphanumeric() {
            self.current += 1;
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        if let Some(keyword) = KEYWORDS.get(value.as_str()) {
            Some(Token::new(keyword.clone(), value, self.line))
        } else {
            Some(Token::new(
                TokenVariant::Identifier(value.clone()),
                value,
                self.line,
            ))
        }
    }
}
