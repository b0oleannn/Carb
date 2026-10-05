use colored::{ColoredString, Colorize};

use crate::frontend::{
    error,
    lexer::token::TokenType,
    parser::{
        binary_expression::parse_binary_expression, identifier::parse_identifier, parser::Parser,
        values::Expression,
    },
};

pub fn parse_expression(parser: &mut Parser) -> Expression {
    return match parser.current_token().token_type {
        TokenType::Letf | TokenType::Let => parse_variable_declaration(parser),

        _ => parse_binary_expression(parser, 0),
    };
}

fn parse_variable_declaration(parser: &mut Parser) -> Expression {
    // let a = expr;
    // let a;
    let is_final: bool = parser.eat().token_type.eq(&TokenType::Letf); // let | letf
    let identifier = parser.eat().value.clone(); // a

    return match parser.eat().value.as_str() {
        "=" => {
            let value = parse_expression(parser);
            Expression::VariableDeclaration {
                is_final,
                identifier,
                value: Box::from(value),
                position: parser.position,
            }
        }
        ";" => {
            if is_final {
                error(ColoredString::from(format!(
                    "Failed to parse variable `{}` declaration at position {}. Final variable must have a value",
                    identifier.bold(),
                    parser.position.saturating_sub(1)
                )));
            }
            Expression::VariableDeclaration {
                is_final: false,
                identifier,
                value: Box::from(Expression::Null(parser.position)),
                position: parser.position,
            }
        }
        _ => error(ColoredString::from(format!(
            "Failed to parse variable `{}` declaration at position {}. Unexpected token {:?}",
            identifier.bold(),
            parser.position.saturating_sub(1),
            parser.current_token()
        ))),
    };
}
pub fn parse_primary_expression(parser: &mut Parser) -> Expression {
    return match parser.current_token().token_type {
        TokenType::Number => Expression::Number(
            parser.current_token().value.parse::<f64>().unwrap(),
            parser.eat().position,
        ),
        TokenType::String => {
            Expression::String(parser.current_token().value.clone(), parser.eat().position)
        }

        TokenType::Identifier => parse_identifier(parser),
        TokenType::OpenBraces => parse_block(parser),

        TokenType::Semicolon => Expression::Null(parser.eat().position),

        TokenType::OpenParenthesis => {
            parser.eat(); // (
            let result = parse_expression(parser); // expr
            parser.expect(TokenType::CloseParenthesis); // )
            result
        }

        _ => error(ColoredString::from(format!(
            "Failed to parse token '{}' at position {}",
            parser.current_token().value.bold().italic(),
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
        body.push(parse_expression(parser));
    }
    parser.eat();
    return Expression::Block(body, parser.current_token().position);
}
