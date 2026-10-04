use crate::frontend::lexer::token::{Token, TokenType};

pub(crate) struct Lexer {
    position: usize,
    chars: Vec<char>,
    src: String,
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        return match self.current_character() {
            '\n' | '\t' => {
                self.eat();
                return Token::ignore_token();
            }
            '-' | '+' | '/' | '*' => {
                return Token::new(
                    TokenType::BinaryOperator,
                    self.eat().to_string(),
                    self.position,
                );
            }

            ' ' => return Token::new(TokenType::WhiteSpace, self.eat().to_string(), self.position),

            _ => {
                if self.current_character().is_numeric() {
                    let start = self.position;
                    //println!("found number at {start}");
                    // passing over the numeric
                    while self.current_character().is_numeric() || self.current_character().eq(&'.')
                    {
                        self.eat();
                    }
                    return Token::new(
                        TokenType::Number,
                        self.src.get(start..self.position).unwrap().to_string(), // using slice to geting the number
                        self.position,
                    );
                }
                Token::bad_token(&self.eat().to_string(), self.position)
            }
        };
    }
    pub fn eat(&mut self) -> char {
        let current_character = self.current_character();
        self.position += 1;
        current_character
    }
    pub fn current_character(&self) -> char {
        return *self.chars.get(self.position).unwrap();
    }

    pub fn new(src: String) -> Self {
        return Self {
            position: 0,
            chars: src.chars().collect(),
            src,
        };
    }

    pub(crate) fn produce_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.position < self.chars.len() {
            let current_token = self.next_token();
            if current_token.token_type.eq(&TokenType::Ignore) {
                continue;
            }
            tokens.push(current_token);
        }
        tokens.push(Token::eof());
        tokens
    }
}
