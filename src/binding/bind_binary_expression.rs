use colored::{ColoredString, Colorize};

use crate::{
    binding::{
        binder::bind_expression,
        error,
        values::{BoundBinaryExpressionType, BoundExpression, ValueType},
    },
    frontend::parser::values::Expression,
};

pub fn bind_binary_expression(
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
    position: usize,
) -> BoundExpression {
    let left_bound = bind_expression(*left.clone());
    let bound_operator = get_binary_operator_type(left.get_type(), &operator, right.get_type());
    let right_bound = bind_expression(*right);
    return BoundExpression::BoundBinaryExpression {
        left: Box::new(left_bound.clone()),
        operator: bound_operator,
        right: Box::new(right_bound),
        value_type: left_bound.get_value_type(),
        position,
    };
}

pub fn get_binary_operator_type(
    left: ValueType,
    operator: &str,
    right: ValueType,
) -> BoundBinaryExpressionType {
    if left.eq(&ValueType::Number) && right.eq(&ValueType::Number) {
        match operator {
            "+" => return BoundBinaryExpressionType::Addition,
            "-" => return BoundBinaryExpressionType::Subtraction,
            "*" => return BoundBinaryExpressionType::Multiplication,
            "/" => return BoundBinaryExpressionType::Division,
            _ => {}
        }
    } else if left.eq(&ValueType::Bool) && right.eq(&ValueType::Bool) {
        match operator {
            "&&" => return BoundBinaryExpressionType::LogicalAnd,
            "||" => return BoundBinaryExpressionType::LogicalOr,
            _ => {}
        }
    }
    match operator {
        "==" => return BoundBinaryExpressionType::Is,
        "!=" => return BoundBinaryExpressionType::IsNot,
        _ => {}
    }
    error(ColoredString::from(format!(
        "Unexpected binary operator {} between `{}` and `{}`",
        operator.bold(),
        left.to_string().bold().yellow(),
        right.to_string().bold().yellow()
    )));
}
