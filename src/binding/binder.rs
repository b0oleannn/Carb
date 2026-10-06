use crate::binding::values::{
    BoundBinaryExpressionType, BoundUnaryOperatorType, LiteralValue, ValueType,
};
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
fn bind_binary_expression(
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

fn get_binary_operator_type(
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
    error(ColoredString::from(format!(
        "Unexpected binary operator {} between `{}` and `{}`",
        operator.bold(),
        left.to_string().bold().yellow(),
        right.to_string().bold().yellow()
    )));
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
