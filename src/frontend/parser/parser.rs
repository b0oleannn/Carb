use crate::frontend::{
    error,
    lexer::{
        lexer::Lexer,
        token::{Token, TokenType},
    },
    parser::{statement::parse_statement, values::Statement},
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
        let tokens = lexer.produce_tokens();
        Self {
            tokens: tokens.clone(),
            position: 0,
            eof_token: tokens.last().unwrap().to_owned(),
        }
    }

    pub fn produce_ast(&mut self) -> Vec<Statement> {
        println!("Tokens: {:?}", self.tokens);
        let mut statements = Vec::new();
        while self.position < self.tokens.len() && !self.is_eof() {
            statements.push(parse_statement(self));
        }
        statements
    }

    pub fn current_token(&self) -> &Token {
        return self.tokens.get(self.position).unwrap_or(&self.eof_token);
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
            "Failed to parse at position {}. Expected {token_type:?}, provided {}",
            current.position.to_string().bold(),
            current.token_type.to_string().yellow(),
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
