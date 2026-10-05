use crate::frontend::parser::{
    expression::parse_primary_expression, parser::Parser, values::Expression,
};

pub fn parse_binary_expression(parser: &mut Parser, parent_precedence: usize) -> Expression {
    let mut left;

    let unary_operator_precedence: usize =
        get_unary_operator_precedence(&parser.current_token().value);

    if unary_operator_precedence >= parent_precedence {
        let operator = parser.eat().value;
        let operand = parse_binary_expression(parser, unary_operator_precedence);
        left = Expression::UnaryExpression {
            operator,
            operand: Box::new(operand),
            position: parser.position,
        }
    } else {
        left = parse_primary_expression(parser);
    }

    loop {
        let operator = parser.current_token().value.clone();
        let precedence = get_binary_operator_precedence(&operator);
        if precedence == 0 || precedence <= parent_precedence {
            break;
        }
        parser.eat();
        let right = parse_binary_expression(parser, precedence);
        left = Expression::BinaryExpression {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
            position: parser.position,
        }
    }
    left
}

fn get_unary_operator_precedence(value: &str) -> usize {
    return match value {
        "+" | "-" => 3,
        _ => 0,
    };
}

fn get_binary_operator_precedence(str: &str) -> usize {
    return match str {
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => 0,
    };
}

// fn parse_multiplicative_expression(parser: &mut Parser) -> Expression {
//     let mut left = expression::parse_primary_expression(parser);
//     while parser.current_token().value.eq("*") || parser.current_token().value.eq("/") {
//         let operator = parser.eat().value.clone();
//         let right = parse_multiplicative_expression(parser);
//         left = Expression::BinaryExpression {
//             left: Box::new(left),
//             operator,
//             right: Box::new(right),
//             position: parser.current_token().position.saturating_sub(1),
//         }
//     }
//     return left;
// }
//
// fn parse_addive_expression(parser: &mut Parser) -> Expression {
//     let mut left = parse_multiplicative_expression(parser);
//     while parser.current_token().value.eq("+") || parser.current_token().value.eq("-") {
//         let operator = parser.eat().value.clone();
//         let right = parse_addive_expression(parser);
//         left = Expression::BinaryExpression {
//             left: Box::new(left),
//             operator,
//             right: Box::new(right),
//             position: parser.current_token().position.saturating_sub(1),
//         }
//     }
//     return left;
// }
