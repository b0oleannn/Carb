use crate::frontend::parser::{
    expression::parse_primary_expression, parser::Parser, values::Expression,
};

pub fn parse_binary_expression(parser: &mut Parser, parent_precedence: usize) -> Expression {
    println!("parsing binary expression:");
    let mut left = parse_primary_expression(parser);
    print!(
        "primary expression: {:?}, next_token: {} \n",
        left,
        parser.current_token().value
    );
    loop {
        let operator = parser.current_token().value.clone();
        let precedence = get_precedence(&operator);
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

fn get_precedence(str: &str) -> usize {
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
