use colored::Colorize;

use crate::{
    binding::values::BoundExpression, evaluator::Program, frontend::parser::values::Expression,
};

mod binary_expression;
mod expression;
mod identifier;
pub mod parser;
pub mod values;

pub fn pretty_print_ast(expressions: Vec<Expression>) {
    println!("{}", "Program".bold().yellow());

    for i in 0..expressions.len() {
        print_expression(
            expressions.get(i).unwrap(),
            false,
            "",
            i == expressions.len() - 1,
        );
    }
}
fn print_expression(expr: &Expression, is_first: bool, prefix: &str, is_last: bool) {
    // ├───
    // ────
    // └───
    let mut marker = "";
    if !is_first {
        marker = if is_last { "└── " } else { "├── " };
    }
    match expr {
        Expression::BinaryExpression {
            left,
            operator,
            right,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "BinaryExpression".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            print_expression(&left, false, &child_prefix, false);
            println!("{child_prefix}├── Operator: {operator:?}");
            print_expression(&right, false, &child_prefix, true);
        }
        Expression::UnaryExpression {
            operator,
            operand,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "UnaryExpression".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            println!("{child_prefix}├── Operator: {operator:?}");
            print_expression(&operand, false, &child_prefix, true);
        }
        Expression::LiteralExpression { value, position: _ } => {
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }

            println!("{prefix}{marker}{}", "LiteralExpression".bold());
            println!("{child_prefix}└── Value: {value:?}");
        }
        Expression::Number(_) => todo!(),
        Expression::String(_) => todo!(),
        Expression::Bool(_) => todo!(),
        Expression::Null => todo!(),

        Expression::Identifier(_, _) => todo!(),
        Expression::Block(expressions, _) => todo!(),
        Expression::VariableDeclaration {
            is_final,
            identifier,
            value,
            position,
        } => todo!(),
    }
}

pub fn pretty_bounded_print(program: &Program) {
    println!("{}", "Program".bold().yellow());

    for i in 0..program.body.len() {
        print_bounded_expression(
            program.body.get(i).unwrap(),
            false,
            "",
            i == program.body.len() - 1,
        );
    }
}
fn print_bounded_expression(expr: &BoundExpression, is_first: bool, prefix: &str, is_last: bool) {
    // ├───
    // ────
    // └───
    let mut marker = "";
    if !is_first {
        marker = if is_last { "└── " } else { "├── " };
    }
    match expr {
        BoundExpression::BoundBinaryExpression {
            left,
            operator,
            right,
            result_type: value_type,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "BinaryExpression".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            print_bounded_expression(&left, false, &child_prefix, false);
            println!("{child_prefix}├── Operator: {operator:?}");
            println!("{child_prefix}├── ValueType: {value_type:?}");
            print_bounded_expression(&right, false, &child_prefix, true);
        }
        BoundExpression::BoundUnaryExpression {
            operator,
            operand,
            value_type,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "UnaryExpression".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            println!("{child_prefix}├── Operator: {operator:?}");
            println!("{child_prefix}├── ValueType: {value_type:?}");
            print_bounded_expression(&operand, false, &child_prefix, true);
        }
        BoundExpression::BoundLiteralExpression {
            value,
            value_type,
            position: _,
        } => {
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }

            println!("{prefix}{marker}{}", "LiteralExpression".bold());
            println!("{child_prefix}├── ValueType: {value_type:?}");
            println!("{child_prefix}└── Value: {value:?}");
        }
    }
}
