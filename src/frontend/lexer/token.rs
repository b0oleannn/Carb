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
    pub fn to_string(self) -> String {
        format!("{:?}", self)
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
    Null,

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

    DoubleEquals,
    NotEquals,

    //WhiteSpace,
    Bad,

    If,
    Else,

    Return,

    Exclamation,
    And,
    Or,

    EOF,
    Ignore,
}

impl TokenType {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
