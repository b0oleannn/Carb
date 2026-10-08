use colored::{ColoredString, Colorize};

use crate::frontend::{
    error,
    lexer::token::TokenType,
    parser::{
        expression::parse_expression,
        parser::Parser,
        values::{Expression, Statement},
    },
};

pub fn parse_statement(parser: &mut Parser) -> Statement {
    match parser.current_token().token_type {
        TokenType::Letf | TokenType::Let => parse_variable_declaration(parser),
        TokenType::Return => parse_return_statement(parser),
        TokenType::Semicolon => {
            parser.eat();
            Statement::Empty
        }
        _ => {
            let expr = parse_expression(parser);

            parser.expect(TokenType::Semicolon);
            Statement::Expression(expr)
        }
    }
}

fn parse_return_statement(parser: &mut Parser) -> Statement {
    parser.eat();
    let value = if parser.current_token().token_type.eq(&TokenType::Semicolon) {
        Option::None
    } else {
        Option::Some(Box::new(parse_expression(parser)))
    };
    parser.expect(TokenType::Semicolon);
    Statement::Return {
        value: value,
        position: parser.position - 1,
    }
}

fn parse_variable_declaration(parser: &mut Parser) -> Statement {
    // let(f) a = <expr>; <- The bind checker has to assign a type based on value. Doesn`t work when the value in null
    // let a; <- Unsupported!
    // let a: Integer = <expr>;
    // let a: Integer;
    let is_final = parser.eat().token_type.eq(&TokenType::Letf);
    let identifier = parser.expect_identifier();
    let mut declared_type = Option::None;
    let value;
    match parser.eat().token_type {
        // : | =
        TokenType::Colon => {
            declared_type = Option::Some(parser.expect_identifier());
            match parser.current_token().token_type {
                TokenType::Equals => {
                    parser.eat();
                    value = parse_expression(parser);
                }
                TokenType::Semicolon => {
                    if is_final {
                        error(ColoredString::from(format!(
                            "Invalid operation at position {}.\n Unable to declare a final variable with null variable",
                            parser.current_token().position.to_string().bold()
                        )))
                    }
                    value = Expression::LiteralExpression {
                        value: Box::from(Expression::Null),
                        position: parser.position,
                    }
                }
                _ => error(ColoredString::from(format!(
                    "Unexpected token during variable declaration at position {}.",
                    parser.current_token().position.to_string().bold()
                ))),
            }
        }
        TokenType::Equals => {
            value = parse_expression(parser);
        }
        TokenType::Semicolon => error(ColoredString::from(format!(
            "Unable to parse the variable declaration at position {}.\n Variable with null value has to have a type",
            parser.current_token().position.to_string().bold()
        ))),
        unsupported => error(ColoredString::from(format!(
            "Unexpected token {} at position {}.\nExpected pattern: let variable_name: type = <value>;",
            unsupported.to_string().bold(),
            parser.current_token().position.to_string().bold()
        ))),
    }
    parser.expect(TokenType::Semicolon);

    return Statement::VariableDeclaration {
        is_final,
        identifier,
        value: value,
        declared_type,
        position: parser.position,
    };
}
