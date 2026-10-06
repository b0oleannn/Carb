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
        _ => Statement::Expression(parse_expression(parser)),
    }
}

fn parse_variable_declaration(parser: &mut Parser) -> Statement {
    // let(f) a = <expr>
    // let a;
    let is_final = parser.eat().token_type.eq(&TokenType::Letf);
    let identifier = parser.expect(TokenType::Identifier).value;
    let value = match parser.current_token().token_type {
        TokenType::Semicolon => {
            if is_final {
                error(ColoredString::from(format!(
                    "Failed to declare final variable {} with null value at position {}",
                    identifier.yellow(),
                    parser.current_token().position.to_string().bold().blue()
                )))
            }
            Expression::LiteralExpression {
                value: Box::new(Expression::Null),
                position: parser.position,
            }
        }
        TokenType::Equals => {
            parser.eat();
            parse_expression(parser)
        }
        _ => error(ColoredString::from(format!(
            "Unexpected token during variable {} declaration at position {}. \n Expected Semicolon or Equals",
            identifier.yellow(),
            parser.current_token().position.to_string().bold().blue()
        ))),
    };
    parser.expect(TokenType::Semicolon);

    return Statement::VariableDeclaration {
        is_final,
        identifier,
        value: value,
        position: parser.position,
    };
}
