use crate::frontend::parser::{parser::Parser, values::Expression};

pub fn parse_identifier(parser: &mut Parser) -> Expression {
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
