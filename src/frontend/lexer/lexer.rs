use std::collections::HashMap;

use colored::{ColoredString, Colorize};

use crate::frontend::{
    error,
    lexer::token::{Token, TokenType},
};

pub struct Lexer {
    position: usize,
    chars: Vec<char>,
    src: String,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn next_token(&mut self) -> Token {
        let token_type;
        let mut pos;
        match self.current_character() {
            '\n' | '\t' | ' ' => {
                token_type = TokenType::Ignore;
                pos = self.get_and_increase_position();
            }
            '-' => {
                pos = self.get_and_increase_position();
                if self.current_character().is_numeric() {
                    token_type = match self.parse_number(true) {
                        NumericVariant::Int(i) => TokenType::Integer(i),
                        NumericVariant::Float(f) => TokenType::Float(f),
                    };
                } else {
                    token_type = TokenType::Minus;
                }
            }
            '+' => {
                pos = self.get_and_increase_position();
                token_type = TokenType::Plus;
            }
            '/' => {
                self.eat();
                if self.current_character().eq(&'/') {
                    self.eat();
                    loop {
                        if self.current_character().eq(&'\n') {
                            break;
                        }
                        self.eat();
                    }
                    token_type = TokenType::Ignore;
                    pos = self.get_and_increase_position();
                } else if self.current_character().eq(&'*') {
                    loop {
                        if self.current_character().eq(&'*') {
                            self.eat();
                            if self.current_character().eq(&'/') {
                                self.eat();
                                break;
                            }
                        }
                        self.eat();
                    }
                    token_type = TokenType::Ignore;
                    pos = self.get_and_increase_position();
                } else {
                    pos = self.position;
                    token_type = TokenType::Slash;
                }
            }
            '*' => {
                pos = self.get_and_increase_position();
                token_type = TokenType::Star;
            }

            '(' => {
                pos = self.get_and_increase_position();
                token_type = TokenType::OpenParenthesis;
            }
            ')' => {
                pos = self.get_and_increase_position();
                token_type = TokenType::CloseParenthesis;
            }
            '{' => {
                pos = self.get_and_increase_position();
                token_type = TokenType::OpenBraces;
            }
            '}' => {
                pos = self.get_and_increase_position();
                token_type = TokenType::CloseBraces;
            }

            '=' => {
                pos = self.get_and_increase_position();
                token_type = if self.current_character().eq(&'=') {
                    pos = self.get_and_increase_position();
                    TokenType::DoubleEquals
                } else {
                    TokenType::Equals
                };
            }

            '"' => {
                token_type = TokenType::String(self.parse_string());
                pos = self.get_and_increase_position();
            }

            ';' => {
                pos = self.get_and_increase_position();
                token_type = TokenType::Semicolon;
            }
            ':' => {
                pos = self.get_and_increase_position();
                token_type = TokenType::Colon;
            }

            '!' => {
                // != | !
                pos = self.get_and_increase_position();
                token_type = if self.current_character().eq(&'=') {
                    pos = self.get_and_increase_position();
                    TokenType::NotEquals
                } else {
                    TokenType::Exclamation
                };
            }
            '&' => {
                pos = self.get_and_increase_position();
                token_type = if self.current_character().eq(&'&') {
                    pos = self.get_and_increase_position();
                    TokenType::DoubleAmpersand
                } else {
                    TokenType::Bad("&".to_string())
                }
            }
            '|' => {
                pos = self.get_and_increase_position();
                token_type = if self.current_character().eq(&'|') {
                    pos = self.get_and_increase_position();
                    TokenType::DoublePipe
                } else {
                    TokenType::Bad("|".to_string())
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                token_type = match self.parse_number(false) {
                    NumericVariant::Int(val) => TokenType::Integer(val),
                    NumericVariant::Float(val) => TokenType::Float(val),
                };
                pos = self.position;
            }
            _ => {
                if self.current_character().is_ascii_alphabetic()
                    || self.current_character().eq(&'_')
                {
                    token_type = self.parse_identifier();
                    pos = self.position;
                } else {
                    pos = self.get_and_increase_position();
                    token_type = TokenType::Bad(self.eat().to_string());
                }
            }
        };
        return Token::new(token_type, pos);
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
                ("return".to_string(), TokenType::Return),
            ]),
        };
    }

    pub fn produce_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.position < self.chars.len() {
            let current_token = self.next_token();
            if current_token.token_type.eq(&TokenType::Ignore) {
                continue;
            }
            tokens.push(current_token);
        }
        tokens.push(Token::eof(self.chars.len().saturating_sub(1)));
        tokens
    }

    // fn peak(&self, offset: usize) -> char {
    //     *self.chars.get(self.position + offset).unwrap()
    // }
    fn get_and_increase_position(&mut self) -> usize {
        self.position += 1;
        return self.position - 1;
    }
    fn parse_string(&mut self) -> String {
        let start = self.position;
        self.position += 1;
        while !self.current_character().eq(&'"') {
            self.position += 1;
        }
        return self.src.get(start..self.position).unwrap().to_owned();
    }

    fn parse_number(&mut self, is_negative: bool) -> NumericVariant {
        // 1000 . 4545

        let start = self.position;
        let mut float_pos: i32 = -1;
        loop {
            match self.current_character() {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    self.position += 1;
                }
                '.' => {
                    if float_pos != -1 {
                        error(ColoredString::from(format!(
                            "Failed to tokenize at position {}. Number unable to have several dots",
                            self.position.to_string().bold(),
                        )))
                    }
                    float_pos = self.get_and_increase_position() as i32;
                }
                _ => {
                    break;
                }
            }
        }
        let end = self.position;
        return if float_pos == -1 {
            NumericVariant::Int({
                let res = self.src.get(start..end).unwrap().parse::<i64>().unwrap();
                if is_negative { -res } else { res }
            })
        } else {
            NumericVariant::Float({
                let res = self.src.get(start..end).unwrap().parse::<f64>().unwrap();
                if is_negative { -res } else { res }
            })
        };
    }

    fn parse_identifier(&mut self) -> TokenType {
        let start = self.position;
        while self.current_character().is_ascii_alphabetic() || self.current_character().eq(&'_') {
            self.position += 1;
        }
        let str = self.src.get(start..self.position).unwrap().to_string();
        if let Some(t) = self.keywords.get(&str) {
            t.to_owned()
        } else {
            TokenType::Identifier(str.to_string())
        }
    }
}
enum NumericVariant {
    Int(i64),
    Float(f64),
}
