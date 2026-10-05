use crate::binding::values::{BoundBinaryExpressionType, BoundUnaryOperatorType, LiteralValue};
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

fn bind_unary_expression(
    operator: String,
    operand: Box<Expression>,
    position: usize,
) -> BoundExpression {
    let bound_operand = bind_expression(*operand);
    let bound_operator: BoundUnaryOperatorType = get_unary_operator_type(&operator);
    return BoundExpression::BoundUnaryExpression {
        operator: bound_operator.clone(),
        operand: Box::new(bound_operand.clone()),
        value_type: bound_operand.get_value_type(),
        position,
    };
}
fn bind_binary_expression(
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
    position: usize,
) -> BoundExpression {
    let left_bound = bind_expression(*left);
    let bound_operator = get_binary_operator_type(&operator);
    let right_bound = bind_expression(*right);
    return BoundExpression::BoundBinaryExpression {
        left: Box::new(left_bound.clone()),
        operator: bound_operator,
        right: Box::new(right_bound),
        value_type: left_bound.get_value_type(),
        position,
    };
}

fn get_binary_operator_type(operator: &str) -> BoundBinaryExpressionType {
    match operator {
        "+" => BoundBinaryExpressionType::Addition,
        "-" => BoundBinaryExpressionType::Substraction,
        "*" => BoundBinaryExpressionType::Multiplication,
        "/" => BoundBinaryExpressionType::Devision,
        _ => error(ColoredString::from(format!(
            "Unexpected bind binary type {}",
            operator.bold()
        ))),
    }
}
pub fn get_unary_operator_type(operator: &str) -> BoundUnaryOperatorType {
    return match operator {
        "+" => BoundUnaryOperatorType::Identity,
        "-" => BoundUnaryOperatorType::Negation,
        _ => error(ColoredString::from(format!(
            "Unexpected bind unary type `{}`",
            operator.bold()
        ))),
    };
}
