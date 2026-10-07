#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub position: usize,
}

impl Token {
    pub fn new(token_type: TokenType, position: usize) -> Self {
        Self {
            token_type,
            position,
        }
    }
    pub fn eof(position: usize) -> Self {
        Self {
            token_type: TokenType::EOF,
            position,
        }
    }
    pub fn ignore_token() -> Self {
        Self {
            token_type: TokenType::Ignore,
            position: 0,
        }
    }
    pub fn bad_token(value: &str, position: usize) -> Self {
        Self::new(TokenType::Bad(value.to_string()), position)
    }
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    String(String),
    Integer(i64),
    Float(f64),
    Identifier(String),

    Let,
    Letf,

    True,
    False,
    Null,

    Plus,
    Minus,
    Slash,
    Star,

    Equals,

    OpenParenthesis,
    CloseParenthesis,
    OpenBraces,
    CloseBraces,
    OpenSquareBraces,
    CloseSquareBraces,

    Semicolon,
    Colon,

    DoubleEquals,
    NotEquals,

    //WhiteSpace,
    Bad(String),

    If,
    Else,

    Return,

    Exclamation,
    DoubleAmpersand,
    DoublePipe,

    EOF,
    Ignore,
}

impl TokenType {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
