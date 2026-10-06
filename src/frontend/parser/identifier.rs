use crate::frontend::{
    lexer::token::TokenType,
    parser::{expression::parse_expression, parser::Parser, values::Expression},
};

pub fn parse_identifier(parser: &mut Parser) -> Expression {
    if parser.peak(1).token_type.eq(&TokenType::Equals) {
        return parse_variable_assignment(parser);
    }

    return Expression::Identifier(
        parser.current_token().value.to_string(),
        parser.eat().position,
    );
}

fn parse_variable_assignment(parser: &mut Parser) -> Expression {
    // a = expr
    let identifier = parser.eat().value;
    parser.expect(TokenType::Equals);
    let expr = parse_expression(parser);
    parser.expect(TokenType::Semicolon);
    Expression::VariableAssignment {
        identifier,
        value: Box::new(expr),
        position: parser.position,
    }
}
