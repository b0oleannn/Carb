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

    return Expression::VariableCall {
        depth: parser.depth,
        identifier,
        position: parser.current_token().position,
    };
}

fn parse_variable_assignment(identifier: String, parser: &mut Parser) -> Expression {
    // a = expr
    parser.expect(TokenType::Equals);
    let expr = parse_expression(parser);
    Expression::VariableAssignment {
        depth: parser.depth,
        identifier,
        value: Box::new(expr),
        position: parser.position,
    }
}
