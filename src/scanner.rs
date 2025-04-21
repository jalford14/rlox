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
            token => error(self.line, format!("Unexpected character: {}", token))
        }
    }

    fn match_token(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() { return '\0' }
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len() as u32
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current as usize).unwrap();
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.push_token(token_type, None);
    }

    fn push_token(&mut self, token_type: TokenType, literal: Option<LiteralValue>) {
        let text = &self.source[self.start as usize..self.current as usize];
        self.tokens.push(Token::new(
            token_type,
            text.to_string(),
            literal,
            self.line
        ));
    }
}
