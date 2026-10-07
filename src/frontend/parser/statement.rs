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
    println!("parsing statement {:?}", parser.current_token());
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
    // let(f) a = <expr>
    // let a;
    let is_final = parser.eat().token_type.eq(&TokenType::Letf);
    let identifier = match parser.eat().token_type {
        TokenType::Identifier(identifier) => identifier,
        v => error(ColoredString::from(format!(
            "Failed to parse variable name {} at position {}",
            v.to_string().yellow(),
            parser.current_token().position.to_string().bold().blue()
        ))),
    };
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
