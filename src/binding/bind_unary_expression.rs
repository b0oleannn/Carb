use colored::{ColoredString, Colorize};

use crate::{
    binding::{
        binder::bind_expression,
        error,
        values::{BoundExpression, BoundUnaryOperatorType, ValueType},
    },
    frontend::parser::values::Expression,
};

pub fn bind_unary_expression(
    operator: String,
    operand: Box<Expression>,
    position: usize,
) -> BoundExpression {
    let bound_operand = bind_expression(*operand);
    let bound_operator: BoundUnaryOperatorType =
        get_unary_operator_type(bound_operand.get_value_type(), &operator);

    BoundExpression::BoundUnaryExpression {
        operator: bound_operator.clone(),
        operand: Box::new(bound_operand.clone()),
        value_type: bound_operand.get_value_type(),
        position,
    }

    // v => error(ColoredString::from(format!(
    //     "Unexpected unary operand. Unary expression is not implemented for {} at position {}",
    //     v.to_string().bold(),
    //     position.to_string().bold()
    // ))),
}
pub fn get_unary_operator_type(value_type: ValueType, operator: &str) -> BoundUnaryOperatorType {
    return match value_type {
        ValueType::Number => match operator {
            "+" => BoundUnaryOperatorType::Identity,
            "-" => BoundUnaryOperatorType::Negation,
            _ => error(ColoredString::from(format!(
                "Unexpected unary operator `{}` for type {}",
                operator.bold(),
                value_type.to_string().bold().yellow(),
            ))),
        },
        ValueType::Bool => match operator {
            "!" => BoundUnaryOperatorType::LogicalNegation,
            _ => error(ColoredString::from(format!(
                "Unexpected unary operator `{}` for type {}",
                operator.bold(),
                value_type.to_string().bold().yellow(),
            ))),
        },
        _ => error(ColoredString::from(format!(
            "Unary operator {} is not implemented for type {}",
            operator.bold(),
            value_type.to_string().bold().yellow(),
        ))),
    };
}
