use colored::Colorize;

use crate::{binding::values::BoundExpression, evaluator::Program};

mod binary_expression;
mod expression;
mod identifier;
pub mod parser;
pub mod values;

pub fn pretty_print(program: &Program) {
    println!("{}", "Program".bold().yellow());

    for i in 0..program.body.len() {
        print_expression(
            program.body.get(i).unwrap(),
            false,
            "",
            i == program.body.len() - 1,
        );
    }
}
fn print_expression(expr: &BoundExpression, is_first: bool, prefix: &str, is_last: bool) {
    // ├───
    // ────
    // └───
    let mut marker = "";
    if !is_first {
        marker = if is_last { "└── " } else { "├── " };
    }
    //     fn print_block(expressions: &Vec<Expression>, is_first: bool, prefix: &str, is_last: bool) {
    //         let mut marker = "";
    //
    //         if !is_first {
    //             marker = if is_last { "└── " } else { "├── " };
    //         }
    //
    //         println!("{prefix}{marker}{}", "Block".bold());
    //
    //         let mut child_prefix = String::new();
    //
    //         if !is_first {
    //             child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
    //         }
    //         for i in 0..expressions.len() {
    //             print_expression(
    //                 expressions.get(i).unwrap(),
    //                 false,
    //                 &child_prefix,
    //                 i == expressions.len() - 1,
    //             );
    //         }
    //     }
    match expr {
        BoundExpression::BoundBinaryExpression {
            left,
            operator,
            right,
            value_type,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "BinaryExpression".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            print_expression(&left, false, &child_prefix, false);
            println!("{child_prefix}├── Operator: {operator:?}");
            println!("{child_prefix}├── ValueType: {value_type:?}");
            print_expression(&right, false, &child_prefix, true);
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
            print_expression(&operand, false, &child_prefix, true);
        }
        BoundExpression::BoundLiteralExpression {
            value,
            value_type,
            position,
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
