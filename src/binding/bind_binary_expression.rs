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
    let right_bound = bind_expression(*right);

    let left_type = left_bound.get_value_type();
    let right_type = right_bound.get_value_type();

    let bound_operator = get_binary_operator_type(left_type, &operator, right_type);
    let result_type: ValueType = get_result_type(left_type, &bound_operator, right_type);
    println!("{result_type:?}");
    return BoundExpression::BoundBinaryExpression {
        left: Box::new(left_bound.clone()),
        operator: bound_operator,
        right: Box::new(right_bound),
        result_type,
        position,
    };
}

fn get_result_type(
    left: ValueType,
    operator: &BoundBinaryExpressionType,
    right: ValueType,
) -> ValueType {
    println!("matching {left:?} {operator:?} {right:?}");
    return match (left, right) {
        (ValueType::Number, ValueType::Number) => match operator {
            BoundBinaryExpressionType::Addition
            | BoundBinaryExpressionType::Subtraction
            | BoundBinaryExpressionType::Multiplication
            | BoundBinaryExpressionType::Division => return ValueType::Number,

            BoundBinaryExpressionType::Is | BoundBinaryExpressionType::IsNot => ValueType::Bool,
            _ => error(ColoredString::from(format!(
                "Not supported. Unable to get result type between `{}` and `{}`",
                left.to_string().yellow(),
                right.to_string().yellow()
            ))),
        },
        (ValueType::Bool, ValueType::Bool) => ValueType::Bool,

        (l, r) => error(ColoredString::from(format!(
            "Unable to get result type between `{}` and `{}`",
            l.to_string().yellow(),
            r.to_string().yellow()
        ))),
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
            "==" => return BoundBinaryExpressionType::Is,
            "!=" => return BoundBinaryExpressionType::IsNot,
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
