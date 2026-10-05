#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub position: usize,
}

impl Token {
    pub fn new(token_type: TokenType, value: String, position: usize) -> Self {
        Self {
            token_type,
            value,
            position,
        }
    }
    pub fn eof() -> Self {
        Self {
            token_type: TokenType::EOF,
            value: "EndOfFile".to_string(),
            position: 0,
        }
    }
    pub fn ignore_token() -> Self {
        Self {
            token_type: TokenType::Ignore,
            value: " ".to_string(),
            position: 0,
        }
    }
    pub fn bad_token(value: &str, position: usize) -> Self {
        Self::new(TokenType::Bad, value.to_string(), position)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    String,
    Number,
    Identifier,

    Let,
    Letf,

    True,
    False,

    BinaryOperator,

    Equals,

    OpenParenthesis,
    CloseParenthesis,
    OpenBraces,
    CloseBraces,
    OpenSquareBraces,
    CloseSquareBraces,

    Semicolon,
    Colon,

    //WhiteSpace,
    Bad,

    If,
    Else,

    EOF,
    Ignore,
}
