use crate::binding::bind_binary_expression::bind_binary_expression;
use crate::binding::bind_unary_expression::bind_unary_expression;
use crate::binding::values::LiteralValue;
use crate::binding::{error, values::BoundExpression};
use crate::frontend::parser::values::Expression;
use colored::{ColoredString, Colorize};

pub fn bind_expression(expr: Expression) -> BoundExpression {
    match expr {
        Expression::BinaryExpression {
            left,
            operator,
            right,
            position,
        } => bind_binary_expression(left, operator, right, position),
        Expression::UnaryExpression {
            operator,
            operand,
            position,
        } => bind_unary_expression(operator, operand, position),
        Expression::LiteralExpression { value, position } => {
            bind_literal_expression(*value, position)
        }
        _ => error(ColoredString::from(format!(
            "Unexpected expression {} for binding type ",
            format!("{:?}", expr).bold()
        ))),
    }
}

fn bind_literal_expression(val: Expression, pos: usize) -> BoundExpression {
    let literal = LiteralValue::from(val);
    return BoundExpression::BoundLiteralExpression {
        value: literal.clone(),
        value_type: literal.get_value_type(),
        position: pos,
    };
}
