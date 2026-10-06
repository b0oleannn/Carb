use std::collections::HashMap;

use crate::frontend::lexer::token::{Token, TokenType};

pub(crate) struct Lexer {
    position: usize,
    chars: Vec<char>,
    src: String,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        return match self.current_character() {
            '\n' | '\t' | ' ' => {
                // skip
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
            '(' => {
                return Token::new(
                    TokenType::OpenParenthesis,
                    self.eat().to_string(),
                    self.position,
                );
            }
            ')' => {
                return Token::new(
                    TokenType::CloseParenthesis,
                    self.eat().to_string(),
                    self.position,
                );
            }
            '{' => {
                return Token::new(TokenType::OpenBraces, self.eat().to_string(), self.position);
            }
            '}' => {
                return Token::new(
                    TokenType::CloseBraces,
                    self.eat().to_string(),
                    self.position,
                );
            }
            '=' => {
                return Token::new(TokenType::Equals, self.eat().to_string(), self.position);
            }
            '"' => {
                let start = self.position;
                self.eat();
                while !self.current_character().eq(&'"') {
                    self.eat();
                }
                let string = self.src.get(start..self.position).unwrap();
                return Token::new(TokenType::String, string.to_string(), self.position);
            }

            ';' => {
                return Token::new(TokenType::Semicolon, self.eat().to_string(), self.position);
            }

            ':' => {
                return Token::new(TokenType::Colon, self.eat().to_string(), self.position);
            }
            _ => {
                if self.current_character().is_numeric() {
                    let start = self.position;
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
                } else if self.current_character().is_ascii_alphabetic()
                    || self.current_character().eq(&'_')
                {
                    let start = self.position;
                    while self.current_character().is_ascii_alphabetic()
                        || self.current_character().eq(&'_')
                    {
                        self.eat();
                    }
                    let str = self.src.get(start..self.position).unwrap().to_string();
                    return Token::new(
                        self.keywords
                            .get(&str)
                            .unwrap_or_else(|| &TokenType::Identifier)
                            .clone(),
                        str,
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
            keywords: HashMap::from([
                ("let".to_string(), TokenType::Let),
                ("letf".to_string(), TokenType::Letf),
                ("true".to_string(), TokenType::True),
                ("false".to_string(), TokenType::False),
                ("null".to_string(), TokenType::Null),
                ("if".to_string(), TokenType::If),
                ("else".to_string(), TokenType::Else),
            ]),
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
