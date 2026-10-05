use crate::frontend::{
    lexer::token::TokenType,
    parser::{expression::parse_expression, parser::Parser, values::Expression},
};

pub fn parse_identifier(parser: &mut Parser) -> Expression {
    if parser.peak(1).token_type.eq(&TokenType::Equals) {
        return parse_variable_assignment(parser);
    }

    return match parser.current_token().value.as_str() {
        "true" => Expression::Bool(true, parser.eat().position),
        "false" => Expression::Bool(false, parser.eat().position),
        "null" => Expression::Null(parser.eat().position),

        _ => {
            return Expression::Identifier(
                parser.current_token().value.to_string(),
                parser.eat().position,
            );
        }
    };
}

fn parse_variable_assignment(parser: &mut Parser) -> Expression {
    // a = expr
    let identifier = parser.eat().value;
    parser.expect(TokenType::Equals);
    let expr = parse_expression(parser);
    Expression::VariableDeclaration {
        is_final: false,
        identifier,
        value: Box::new(expr),
    }
}
