use crate::frontend::{
    lexer::token::TokenType::Identifier,
    parser::expression::{Expression, Statement},
};

use colored::Colorize;

pub mod expression;
pub mod parser;

pub fn pretty_print(statement: Statement) {
    match statement {
        Statement::Expr(expr) => {
            print_expression(&expr, true, "", false);
        }
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
        Expression::Number(num) => {
            println!("{prefix}{marker}Number: {num}")
        }
        Expression::BinaryExpression {
            left,
            operator,
            right,
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
    }
}
