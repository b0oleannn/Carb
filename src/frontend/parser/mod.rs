use colored::Colorize;

use crate::{
    binding::values::{BoundExpression, BoundStatement},
    frontend::parser::values::{Expression, Statement},
    runtime::values::Program,
};

mod binary_expression;
mod expression;
mod identifier;
pub mod parser;
mod statement;
pub mod values;

pub fn pretty_print_ast(statements: &Vec<Statement>) {
    println!("{}", "Program".bold().yellow());

    for i in 0..statements.len() {
        print_statement(
            statements.get(i).unwrap(),
            false,
            "",
            i == statements.len() - 1,
        );
    }
}

pub fn print_statement(statement: &Statement, is_first: bool, prefix: &str, is_last: bool) {
    let mut marker = "";
    if !is_first {
        marker = if is_last { "└── " } else { "├── " };
    }
    match statement {
        Statement::VariableDeclaration {
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
            println!("{child_prefix}├── IsFinal: {is_final:?}");
            println!("{child_prefix}├── Identifier: {identifier:?}");
            print_expression(&value, false, &child_prefix, true);
        }
        Statement::Expression(expression) => print_expression(expression, false, prefix, is_last),
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
        Expression::VariableAssignment {
            identifier,
            value,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "VariableAssignment".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            println!("{child_prefix}├── Identifier: {identifier:?}");
            print_expression(&value, false, &child_prefix, true);
        }
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

        Expression::Block(expressions, _) => {
            println!("{prefix}{marker}{}", "Block".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            for i in 0..expressions.len() {
                print_statement(
                    expressions.get(i).unwrap(),
                    false,
                    &child_prefix,
                    i == expressions.len() - 1,
                );
            }
        }

        Expression::Return { value, position: _ } => {
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }

            println!("{prefix}{marker}{}", "Return".bold());
            match value {
                Some(v) => {
                    print_expression(&v, false, &child_prefix, true);
                }
                None => println!("{child_prefix}└── Void"),
            }
        }

        Expression::Number(val) => println!("{prefix}{marker}Number : {val:?}",),
        Expression::String(val) => println!("{prefix}{marker}String : {val:?}",),
        Expression::Bool(val) => println!("{prefix}{marker}Bool : {val:?}",),
        Expression::Null => println!("{prefix}{marker}{}", "Null".bold()),

        Expression::Identifier(_, _) => todo!(),
    }
}

pub fn pretty_bounded_print(program: &Program) {
    println!("{}", "Program".bold().yellow());

    for i in 0..program.body.len() {
        print_bounded_statement(
            program.body.get(i).unwrap(),
            false,
            "",
            i == program.body.len() - 1,
        );
    }
}
pub fn print_bounded_statement(
    statement: &BoundStatement,
    is_first: bool,
    prefix: &str,
    is_last: bool,
) {
    let mut marker = "";
    if !is_first {
        marker = if is_last { "└── " } else { "├── " };
    }
    match statement {
        BoundStatement::BoundVariableDeclaration {
            is_final,
            identifier,
            value,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "BoundVariableDeclaration".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            println!("{child_prefix}├── IsFinal: {is_final:?}");
            println!("{child_prefix}├── Identifier: {identifier:?}");
            print_bound_expression(&value, false, &child_prefix, true);
        }
        BoundStatement::BoundExpression(bound_expression) => {
            print_bound_expression(bound_expression, is_first, prefix, is_last)
        }
    }
}
fn print_bound_expression(expr: &BoundExpression, is_first: bool, prefix: &str, is_last: bool) {
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
            println!("{prefix}{marker}{}", "BoundBinaryExpression".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            print_bound_expression(&left, false, &child_prefix, false);
            println!("{child_prefix}├── Operator: {operator:?}");
            println!("{child_prefix}├── ValueType: {value_type:?}");
            print_bound_expression(&right, false, &child_prefix, true);
        }
        BoundExpression::BoundUnaryExpression {
            operator,
            operand,
            value_type,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "BoundUnaryExpression".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            println!("{child_prefix}├── Operator: {operator:?}");
            println!("{child_prefix}├── ValueType: {value_type:?}");
            print_bound_expression(&operand, false, &child_prefix, true);
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

            println!("{prefix}{marker}{}", "BoundLiteralExpression".bold());
            println!("{child_prefix}├── ValueType: {value_type:?}");
            println!("{child_prefix}└── Value: {value:?}");
        }
        BoundExpression::BoundReturn {
            value,
            value_type,
            position: _,
        } => {
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }

            println!("{prefix}{marker}{}", "BoundReturn".bold());
            match value {
                Some(v) => {
                    println!("{child_prefix}├── Type: {value_type:?}");
                    print_bound_expression(&v, false, &child_prefix, true);
                }
                None => println!("{child_prefix}└── Void"),
            }
        }

        BoundExpression::BoundBlock {
            bounded_statements: statements,
            value_type,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "BoundBlock".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            println!("{child_prefix}├── Type: {value_type:?}");

            for i in 0..statements.len() {
                print_bounded_statement(
                    statements.get(i).unwrap(),
                    false,
                    &child_prefix,
                    i == statements.len() - 1,
                );
            }
        }
        BoundExpression::BoundVariableAssignment {
            identifier,
            value,
            value_type,
            position: _,
        } => {
            println!("{prefix}{marker}{}", "VariableAssignment".bold());
            let mut child_prefix = String::new();
            if !is_first {
                child_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            }
            println!("{child_prefix}├── Identifier: {identifier:?}");
            println!("{child_prefix}├── ValueType: {value_type:?}");
            print_bound_expression(&value, false, &child_prefix, true);
        }
    }
}
