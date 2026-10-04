use crate::frontend::{
    error,
    lexer::{
        lexer::Lexer,
        token::{Token, TokenType},
    },
    parser::expression::{Expression, Statement},
};

use colored::{ColoredString, Colorize};

pub(crate) struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(source: String) -> Self {
        let mut lexer = Lexer::new(source);

        Self {
            tokens: lexer.produce_tokens(),
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.position < self.tokens.len() && !self.is_eof() {
            statements.push(self.parse_statement());
        }
        statements
    }

    pub fn parse_statement(&mut self) -> Statement {
        return match self.current_token().token_type {
            _ => Statement::Expr(self.parse_expression()),
        };
    }

    pub fn parse_expression(&mut self) -> Expression {
        return self.parse_binary_expression();
    }
    fn parse_primary_expression(&mut self) -> Expression {
        return match self.current_token().token_type {
            TokenType::Number => Expression::Number(self.eat().value.parse::<f64>().unwrap()),
            TokenType::OpenParenthesis => {
                self.eat(); // (
                let result = self.parse_expression(); // expr
                self.expect(TokenType::CloseParenthesis); // )
                result
            }
            _ => error(colored::ColoredString::from(format!(
                "Failed to parse token type '{}' at position {}",
                self.current_token().value.bold().italic(),
                self.current_token().position.to_string().bold()
            ))),
        };
    }

    fn parse_binary_expression(&mut self) -> Expression {
        return self.parse_addive_expression();
    }

    fn parse_multiplicative_expression(&mut self) -> Expression {
        let mut left = self.parse_primary_expression();
        while self.current_token().value.eq("*") || self.current_token().value.eq("/") {
            let operator = self.eat().value.clone();
            let right = self.parse_multiplicative_expression();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }
        return left;
    }

    fn parse_addive_expression(&mut self) -> Expression {
        let mut left = self.parse_multiplicative_expression();
        while self.current_token().value.eq("+") || self.current_token().value.eq("-") {
            let operator = self.eat().value.clone();
            let right = self.parse_addive_expression();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            }
        }
        return left;
    }

    pub fn current_token(&self) -> &Token {
        return self.tokens.get(self.position).unwrap();
    }
    pub fn is_eof(&self) -> bool {
        self.current_token().token_type.eq(&TokenType::EOF)
    }
    pub fn expect(&mut self, token_type: TokenType) -> &Token {
        let current = self.current_token();
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

    fn eat(&mut self) -> &Token {
        if let Some(current) = self.tokens.get(self.position) {
            self.position += 1;
            return current;
        }
        error(ColoredString::from("Failed to jump to the next token"));
    }
}
