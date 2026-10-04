#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Token {
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
        Self::new(TokenType::EOF, "EndOfFile".to_string(), 0)
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
pub(crate) enum TokenType {
    String,
    Number,
    Identifier,

    BinaryOperator,

    Equals,

    OpenParenthesis,
    CloseParenthesis,
    OpenBraces,
    CloseBraces,
    OpenSquareBraces,
    CloseSquareBraces,
    //WhiteSpace,
    Bad,

    EOF,
    Ignore,
}
