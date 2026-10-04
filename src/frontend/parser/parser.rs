use crate::frontend::{
    error,
    lexer::{
        lexer::Lexer,
        token::{Token, TokenType},
    },
    parser::{expression::parse_expression, values::Program},
};

use colored::{ColoredString, Colorize};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    pub position: usize,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let mut lexer = Lexer::new(source);

        Self {
            tokens: lexer.produce_tokens(),
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Program {
        let mut statements = Vec::new();
        while self.position < self.tokens.len() && !self.is_eof() {
            statements.push(parse_expression(self));
        }
        Program::new(statements)
    }

    pub fn current_token(&self) -> &Token {
        return self.tokens.get(self.position).unwrap();
    }
    pub fn is_eof(&self) -> bool {
        self.current_token().token_type.eq(&TokenType::EOF)
    }
    pub fn expect(&mut self, token_type: TokenType) -> &Token {
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

    pub fn eat(&mut self) -> &Token {
        if let Some(current) = self.tokens.get(self.position) {
            self.position += 1;
            return current;
        }
        error(ColoredString::from("Failed to jump to the next token"));
    }
}
