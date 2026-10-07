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
    println!("parsing primary expression {:?}", parser.current_token());
    return match parser.current_token().token_type.clone() {
        TokenType::Integer(int) => Expression::LiteralExpression {
            value: Box::new(Expression::Integer(int)),
            position: parser.eat().position,
        },
        TokenType::Float(float) => Expression::LiteralExpression {
            value: Box::new(Expression::Float(float)),
            position: parser.eat().position,
        },
        TokenType::String(val) => Expression::LiteralExpression {
            value: Box::new(Expression::String(val)),
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

        TokenType::Identifier(_) => parse_identifier(parser),

        ref unrecognized => error(ColoredString::from(format!(
            "Failed to parse token {} at position {}",
            unrecognized.to_string().bold(),
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
    println!("{:?}", parser.eat());
    return Expression::Block(body, parser.current_token().position);
}
