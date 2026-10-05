use crate::frontend::parser::values::{Expression, Program};

use colored::Colorize;

mod binary_expression;
mod expression;
mod identifier;
pub mod parser;
pub mod values;

pub fn pretty_print(program: Program) {
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
fn print_expression(expr: &Expression, is_first: bool, prefix: &str, is_last: bool) {
    // ├───
    // ────
    // └───
    let mut marker = "";
    if !is_first {
        marker = if is_last { "└── " } else { "├── " };
    }
    fn print_block(expressions: &Vec<Expression>, is_first: bool, prefix: &str, is_last: bool) {
        let mut marker = "";

        if !is_first {
            marker = if is_last { "└── " } else { "├── " };
        }

        println!("{prefix}{marker}{}", "Block".bold());

        let mut child_prefix = String::new();

        if !is_first {
            child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
        }
        for i in 0..expressions.len() {
            print_expression(
                expressions.get(i).unwrap(),
                false,
                &child_prefix,
                i == expressions.len() - 1,
            );
        }
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
            println!("{child_prefix}├── Operator: {operator}");
            print_expression(&right, false, &child_prefix, true);
        }
        Expression::VariableDeclaration {
            is_final,
            identifier,
            value,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "VariableDeclaration".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            println!("{child_prefix}├── IsFinal: {is_final}");
            println!("{child_prefix}├── Identifier: {identifier}");
            print_expression(&value, false, &child_prefix, true);
        }

        Expression::String(str, _pos) => println!("{prefix}{marker}String: {str}"),
        Expression::Bool(val, _pos) => println!("{prefix}{marker}Bool: {val}"),
        Expression::Null(_pos) => println!("{prefix}{marker}Null"),
        Expression::Number(num, _pos) => println!("{prefix}{marker}Number: {num}"),

        Expression::Identifier(identifier, _pos) => {
            println!("{prefix}{marker}Number: {identifier}")
        }

        Expression::Block(statements, _pos) => print_block(statements, is_first, prefix, is_last),
    }
}
