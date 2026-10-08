use crate::frontend::{
    lexer::token::TokenType,
    parser::{expression::parse_expression, parser::Parser, values::Expression},
};

pub fn parse_identifier(parser: &mut Parser) -> Expression {
    // identifier = <value> ;
    let identifier = match parser.eat().token_type {
        TokenType::Identifier(str) => str,
        _ => unreachable!(),
    };
    if parser.current_token().token_type.eq(&TokenType::Equals) {
        return parse_variable_assignment(identifier, parser);
    }

    return Expression::VariableCall(identifier, parser.current_token().position);
}

fn parse_variable_assignment(identifier: String, parser: &mut Parser) -> Expression {
    // a = expr
    parser.expect(TokenType::Equals);
    let expr = parse_expression(parser);
    Expression::VariableAssignment {
        identifier,
        value: Box::new(expr),
        position: parser.position,
    }
}
