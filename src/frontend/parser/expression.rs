use colored::{ColoredString, Colorize};

use crate::frontend::{
    error,
    lexer::token::TokenType,
    parser::{
        binary_expression::parse_binary_expression, identifier::parse_identifier, parser::Parser,
        statement::parse_statement, values::Expression,
    },
};

pub fn parse_expression(parser: &mut Parser) -> Expression {
    parse_binary_expression(parser, 0)
}

pub fn parse_primary_expression(parser: &mut Parser) -> Expression {
    return match parser.current_token().token_type {
        TokenType::Number => Expression::LiteralExpression {
            value: Box::new(Expression::Number(
                parser.current_token().value.parse::<f64>().unwrap(),
            )),
            position: parser.eat().position,
        },
        TokenType::String => Expression::LiteralExpression {
            value: Box::new(Expression::String(parser.current_token().value.clone())),
            position: parser.eat().position,
        },
        TokenType::True => Expression::LiteralExpression {
            value: Box::new(Expression::Bool(true)),
            position: parser.eat().position,
        },
        TokenType::False => Expression::LiteralExpression {
            value: Box::new(Expression::Bool(false)),
            position: parser.eat().position,
        },
        TokenType::Null => Expression::LiteralExpression {
            value: Box::new(Expression::Null),
            position: parser.eat().position,
        },

        TokenType::OpenParenthesis => {
            parser.eat(); // (
            let result = parse_expression(parser); // expr
            parser.expect(TokenType::CloseParenthesis); // )
            result
        }
        TokenType::OpenBraces => parse_block(parser),
        TokenType::Return => {
            parser.eat();
            let value = if parser.current_token().token_type.eq(&TokenType::Semicolon) {
                Option::None
            } else {
                Option::Some(Box::new(parse_expression(parser)))
            };
            parser.expect(TokenType::Semicolon);

            Expression::Return {
                value: value,
                position: parser.position - 1,
            }
        }
        TokenType::Identifier => parse_identifier(parser),

        _ => error(ColoredString::from(format!(
            "Failed to parse token {} : '{}' at position {}",
            parser.current_token().token_type.to_string().bold(),
            parser.current_token().value.bold().yellow(),
            parser.current_token().position.to_string().bold()
        ))),
    };
}

fn parse_block(parser: &mut Parser) -> Expression {
    let mut body = Vec::new();
    parser.eat(); // {

    while !parser
        .current_token()
        .token_type
        .eq(&TokenType::CloseBraces)
        && !parser.is_eof()
    {
        body.push(parse_statement(parser));
    }
    parser.eat();
    return Expression::Block(body, parser.current_token().position);
}
