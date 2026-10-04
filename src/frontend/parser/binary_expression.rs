use crate::frontend::parser::{expression, parser::Parser, values::Expression};

pub fn parse_binary_expression(parser: &mut Parser) -> Expression {
    return parse_addive_expression(parser);
}

fn parse_multiplicative_expression(parser: &mut Parser) -> Expression {
    let mut left = expression::parse_primary_expression(parser);
    while parser.current_token().value.eq("*") || parser.current_token().value.eq("/") {
        let operator = parser.eat().value.clone();
        let right = parse_multiplicative_expression(parser);
        left = Expression::BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            position: parser.current_token().position.saturating_sub(1),
        }
    }
    return left;
}

fn parse_addive_expression(parser: &mut Parser) -> Expression {
    let mut left = parse_multiplicative_expression(parser);
    while parser.current_token().value.eq("+") || parser.current_token().value.eq("-") {
        let operator = parser.eat().value.clone();
        let right = parse_addive_expression(parser);
        left = Expression::BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
            position: parser.current_token().position.saturating_sub(1),
        }
    }
    return left;
}
