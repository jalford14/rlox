use std::collections::HashMap;
use crate::token::{Token, TokenType, LiteralValue};
use crate::error;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: u32,
    pub current: u32,
    pub line: u32
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end()  {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line
        ));
        
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        let match_token: bool = self.match_token('=');
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_token(if match_token { TokenType::BangEqual } else { TokenType::Bang }),
            '=' => self.add_token(if match_token { TokenType::EqualEqual } else { TokenType::Equal }),
            '<' => self.add_token(if match_token { TokenType::LessEqual } else { TokenType::Less }),
            '>' => self.add_token(if match_token { TokenType::GreaterEqual } else { TokenType::Greater }),
            '/' => {
                if self.match_token('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            token => {
                if self.is_digit(token) {
                    self.number()
                }  else if self.is_alpha(token) {
                    self.identifier();
                }
                else {
                    error(self.line, format!("Unexpected character: {}", token))
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) { self.advance(); }

        let text: String = self.source.chars()
            .skip(self.start as usize)
            .take((self.current - self.start) as usize)
            .collect();
        let token_type: Option<TokenType> = self.keywords().get(text.as_str()).cloned();

        match token_type {
            None => self.add_token(TokenType::Identifier),
            Some(val) => self.add_token(val),
        }
    }

    fn number(&mut self) {
        self.consume_numbers();

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            self.consume_numbers();
        }

        let value: String = self.source.chars()
            .take((self.current - self.start) as usize).collect();
        let as_double: f64 = value.parse().unwrap();
        self.add_token_with_value(TokenType::Number, Some(LiteralValue::Number(as_double)));
    }

    fn consume_numbers(&mut self) {
        loop {
            let peek: char = self.peek();
            if self.is_digit(peek) { self.advance(); } else { break; }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, String::from("Unterminated string."));
            return;
        }

        self.advance();

        let value: String = self.source.chars().skip(self.start as usize + 1)
            .take((self.current - self.start - 2) as usize).collect();
        self.add_token_with_value(TokenType::String, Some(LiteralValue::String(value)));
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0' }
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.chars().count().try_into().unwrap() { return '\0' }
        return self.source.chars().nth((self.current + 1) as usize).unwrap();
    }

    fn is_alpha(&self, c: char) -> bool {
        self.is_alpha_numeric(c) || self.is_digit(c)
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as u32
    }

    fn advance(&mut self) -> char {
        match self.source.chars().nth(self.current as usize) {
            None => '\0',
            Some(c) => {
                self.current += 1;
                return c;
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_value(token_type, None);
    }

    fn add_token_with_value(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text: String = self.source.chars().skip(self.start as usize)
            .take((self.current - self.start) as usize).collect();
        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            literal,
            self.line
        ));
    }

    fn keywords(&self) -> HashMap<&str, TokenType> {
        HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While)
        ])
    }
}
