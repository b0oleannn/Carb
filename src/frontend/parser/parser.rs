use crate::frontend::{
    error,
    lexer::{
        lexer::Lexer,
        token::{Token, TokenType},
    },
    parser::{expression::parse_expression, values::Expression},
};

use colored::{ColoredString, Colorize};

pub struct Parser {
    tokens: Vec<Token>,
    pub position: usize,
    eof_token: Token,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let mut lexer = Lexer::new(source);

        Self {
            tokens: lexer.produce_tokens(),
            position: 0,
            eof_token: Token::eof(),
        }
    }

    pub fn produce_ast(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::new();
        while self.position < self.tokens.len() && !self.is_eof() {
            expressions.push(parse_expression(self));
        }
        expressions
    }

    pub fn current_token(&self) -> &Token {
        return self.tokens.get(self.position).unwrap();
    }
    pub fn is_eof(&self) -> bool {
        self.current_token().token_type.eq(&TokenType::EOF)
    }
    pub fn expect(&mut self, token_type: TokenType) -> Token {
        let current = self.eat();
        if current.token_type.eq(&token_type) {
            return current;
        }
        error(ColoredString::from(format!(
            "Failed to parse at position {}. Expected {token_type:?}, provided {:?}:{}",
            current.position.to_string().bold(),
            current.token_type,
            current.value.bold()
        )))
    }

    pub fn peak(&self, offset: usize) -> &Token {
        if let Some(token) = self.tokens.get(self.position + offset) {
            return token;
        }
        return &self.eof_token;
    }

    pub fn eat(&mut self) -> Token {
        if let Some(current) = self.tokens.get(self.position) {
            self.position += 1;
            return current.to_owned();
        }
        error(ColoredString::from(format!(
            "Failed to jump to the next token. {}/{}",
            self.position,
            self.tokens.len() - 1,
        )));
    }
}
