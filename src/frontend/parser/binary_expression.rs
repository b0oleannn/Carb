use colored::{ColoredString, Colorize};

use crate::frontend::{
    error,
    lexer::token::TokenType,
    parser::{expression::parse_primary_expression, parser::Parser, values::Expression},
};

pub fn parse_binary_expression(parser: &mut Parser, parent_precedence: usize) -> Expression {
    println!("Parsing binary expression {:?}", parser.current_token());
    let mut left;

    let unary_operator_precedence: usize =
        get_unary_operator_precedence(&parser.current_token().token_type);

    if unary_operator_precedence != 0 && unary_operator_precedence >= parent_precedence {
        let operator = match parser.eat().token_type {
            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Exclamation => "!",
            unrecognized => error(ColoredString::from(format!(
                "Failed to get operator from token {} at position {}",
                unrecognized.to_string().bold(),
                parser.current_token().position.to_string().bold()
            ))),
        }
        .to_string();
        let operand = parse_binary_expression(parser, unary_operator_precedence);
        left = Expression::UnaryExpression {
            operator,
            operand: Box::new(operand),
            position: parser.position,
        }
    } else {
        left = parse_primary_expression(parser);
        println!("Left : {:?}", left);
    }

    loop {
        let precedence = get_binary_operator_precedence(&parser.current_token().token_type);
        if precedence == 0 || precedence <= parent_precedence {
            break;
        }
        let operator_token = parser.eat();
        let operator = match operator_token.clone().token_type {
            TokenType::Minus => "-",
            TokenType::Plus => "+",
            TokenType::Star => "*",
            TokenType::Slash => "/",
            TokenType::DoubleAmpersand => "&&",
            TokenType::DoublePipe => "||",
            TokenType::DoubleEquals => "==",
            TokenType::NotEquals => "!=",
            _unrecognized => unreachable!(),
        };
        let right = parse_binary_expression(parser, precedence);
        left = Expression::BinaryExpression {
            left: Box::new(left),
            operator: operator.to_owned(),
            right: Box::new(right),
            position: parser.position,
        }
    }
    left
}

fn get_unary_operator_precedence(token_type: &TokenType) -> usize {
    return match token_type {
        TokenType::Plus | TokenType::Minus | TokenType::Exclamation => 6,
        _ => 0,
    };
}

fn get_binary_operator_precedence(token_type: &TokenType) -> usize {
    return match token_type {
        TokenType::Star | TokenType::Slash => 5,
        TokenType::Plus | TokenType::Minus => 4,
        TokenType::DoubleEquals | TokenType::NotEquals => 3,
        TokenType::DoubleAmpersand => 2,
        TokenType::DoublePipe => 1,
        _ => 0,
    };
}
