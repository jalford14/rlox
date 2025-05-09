#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Plus,
    Minus,
    Semicolon,
    Slash,
    Star,
    
    // One or two character tokens
    Bang,
    BangEqual,    // !=
    Equal,
    EqualEqual,   // ==
    Greater,
    GreaterEqual, // >=
    Less,
    LessEqual,    // <=
    
    // Literals
    Identifier,
    String,
    Number,
    
    // Keywords
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
    
    // Special tokens
    Eof
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line: u32,
}

// For holding literal values
#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line: u32) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}


