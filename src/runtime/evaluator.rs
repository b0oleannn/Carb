use std::process::exit;

use colored::{ColoredString, Colorize};

use crate::{
    binding::values::{
        BoundBinaryExpressionType, BoundExpression, BoundStatement, BoundUnaryOperatorType,
        LiteralValue, ValueType,
    },
    runtime::values::{Block, Program, RuntimeValue},
};

pub fn evaluate_program(program: Program) {
    let mut program_block = Block::program_block();
    for statement in program.body {
        println!("{:?}", evaluate_statement(statement, &mut program_block));
    }
}
fn evaluate_statement(statement: BoundStatement, block: &mut Block) -> Option<RuntimeValue> {
    match statement {
        BoundStatement::BoundVariableDeclaration {
            is_final,
            identifier,
            value,
            position,
        } => {
            evaluate_variable_declaration(is_final, identifier, value, position, block);
            Option::None
        }
        BoundStatement::BoundExpression(bound_expression) => {
            Option::from(evaluate_expression(bound_expression, block))
        }
    }
}

fn evaluate_variable_declaration(
    is_final: bool,
    identifier: String,
    value: Box<BoundExpression>,
    position: usize,
    block: &mut Block,
) {
    let val = evaluate_expression(*value, block);
    match block.declare_variable(is_final, &identifier, val) {
        Ok(v) => {
            println!("Declared variable {identifier}: {v:?}")
        }
        Err(msg) => error(ColoredString::from(format!(
            "{msg} at position `{}`",
            position.to_string().bold().blue(),
        ))),
    }
}
fn evaluate_expression(expression: BoundExpression, block: &mut Block) -> RuntimeValue {
    return match expression {
        BoundExpression::BoundLiteralExpression {
            value,
            value_type: _,
            position,
        } => {
            return match value {
                LiteralValue::Number(num) => RuntimeValue::Number(num),
                LiteralValue::Bool(b) => RuntimeValue::Bool(b),
                LiteralValue::String(str) => RuntimeValue::String(str),
                LiteralValue::Null => RuntimeValue::Null,

                _val => error(ColoredString::from(format!(
                    "Failed to evaluate value {} at position {}",
                    format!("{_val:?}").bold(),
                    position.to_string().bold().blue(),
                ))),
            };
        }
        BoundExpression::BoundVariableAssignment {
            identifier,
            value,
            value_type,
            position,
        } => evaluate_variable_assignment(identifier, value, value_type, position, block),
        BoundExpression::BoundUnaryExpression {
            operator,
            operand,
            value_type,
            position,
        } => evaluate_unary_expression((operator, operand), value_type, block, position),

        BoundExpression::BoundBinaryExpression {
            left,
            operator,
            right,
            result_type,
            position,
        } => evaluate_binary_expression(*left, operator, *right, result_type, position, block),
        _val => error(ColoredString::from(format!(
            "{} is not implemented yet",
            format!("{_val:?}").bold().blue()
        ))),
    };
}

fn evaluate_variable_assignment(
    identifier: String,
    value: Box<BoundExpression>,
    value_type: ValueType,
    position: usize,
    block: &mut Block,
) -> RuntimeValue {
    let val = evaluate_expression(*value, block);
    if let Some(stored_val) = block.inspect_variable(&identifier) {
        if stored_val.is_null()
            || value_type.eq(&ValueType::Null)
            || stored_val.get_value_type().eq(&value_type)
        {
            match block.assign_variable(&identifier, val) {
                Ok(run) => return run,
                Err(err) => {
                    error(ColoredString::from(format!(
                        "{err} at position {}",
                        position.to_string().bold().blue()
                    )));
                }
            }
        } else {
            error(ColoredString::from(format!(
                "Type mismatch between value {}: {} and variable {}: {} ({}) at position {}",
                val.to_string().bold().yellow(),
                value_type.to_string().bold().green(),
                identifier.to_string().bold().yellow(),
                stored_val.clone().to_string().bold().white(),
                stored_val
                    .get_value_type()
                    .clone()
                    .to_string()
                    .bold()
                    .green(),
                position.to_string().bold().blue()
            )));
        }
    } else {
        error(ColoredString::from(format!(
            "Variable {} wasn`t found in the block at position {}",
            identifier.to_string().bold().yellow(),
            position.to_string().bold().blue()
        )));
    }
}

fn evaluate_unary_expression(
    unary_expression: (BoundUnaryOperatorType, Box<BoundExpression>),
    _result_type: ValueType,
    block: &mut Block,
    _position: usize,
) -> RuntimeValue {
    match unary_expression.0 {
        BoundUnaryOperatorType::Identity => evaluate_expression(*unary_expression.1, block),
        BoundUnaryOperatorType::Negation => RuntimeValue::Number(
            -evaluate_expression(*unary_expression.1, block).expect_value_type::<f64>(),
        ),
        BoundUnaryOperatorType::LogicalNegation => RuntimeValue::Bool(
            !evaluate_expression(*unary_expression.1, block).expect_value_type::<bool>(),
        ),
    }
}

fn evaluate_binary_expression(
    left: BoundExpression,
    operator: BoundBinaryExpressionType,
    right: BoundExpression,
    result_type: ValueType,
    position: usize,
    block: &mut Block,
) -> RuntimeValue {
    let left = evaluate_expression(left, block);
    let right = evaluate_expression(right, block);

    match result_type {
        ValueType::Number => {
            return evaluate_number_binary_expression(left, right, operator, position);
        }
        ValueType::Bool => return evaluate_bool_binary_expression(left, right, operator, position),
        _ => error(ColoredString::from(format!(
            "Binary expression in not implemented for {} {} {} at position {}",
            left.to_string().bold(),
            format!("{:?}", operator).bold().yellow(),
            right.to_string().bold(),
            position.to_string().bold().blue()
        ))),
    }
}

fn evaluate_bool_binary_expression(
    left: RuntimeValue,
    right: RuntimeValue,
    operator: BoundBinaryExpressionType,
    position: usize,
) -> RuntimeValue {
    let left = left.expect_value_type::<bool>();
    let right = right.expect_value_type::<bool>();
    return RuntimeValue::Bool(match operator {
        BoundBinaryExpressionType::LogicalAnd => left && right,
        BoundBinaryExpressionType::LogicalOr => left || right,
        BoundBinaryExpressionType::Is => left == right,
        BoundBinaryExpressionType::IsNot => left != right,
        v => error(ColoredString::from(format!(
            "Binary expression in not implemented for {} {} {} at position {}",
            left.to_string().bold(),
            format!("{:?}", v).bold().yellow(),
            right.to_string().bold(),
            position.to_string().bold().blue()
        ))),
    });
}

fn evaluate_number_binary_expression(
    left: RuntimeValue,
    right: RuntimeValue,
    operator: BoundBinaryExpressionType,
    position: usize,
) -> RuntimeValue {
    let left = left.expect_value_type::<f64>();
    let right = right.expect_value_type::<f64>();
    return match operator {
        BoundBinaryExpressionType::Addition => RuntimeValue::Number(left + right),
        BoundBinaryExpressionType::Subtraction => RuntimeValue::Number(left - right),
        BoundBinaryExpressionType::Multiplication => RuntimeValue::Number(left * right),
        BoundBinaryExpressionType::Division => RuntimeValue::Number(left / right),
        BoundBinaryExpressionType::Is => RuntimeValue::Bool(left == right),
        BoundBinaryExpressionType::IsNot => RuntimeValue::Bool(left != right),
        v => error(ColoredString::from(format!(
            "Binary expression in not implemented for {} {} {} at position {}",
            left.to_string().bold(),
            format!("{:?}", v).bold().yellow(),
            right.to_string().bold(),
            position.to_string().bold().blue()
        ))),
    };
}

pub fn error(message: ColoredString) -> ! {
    println!(
        " \n =====  \n {} \n {message} \n ===== ",
        "Evaluation Error".red().bold(),
    );
    exit(-1);
}

impl TryFrom<RuntimeValue> for bool {
    type Error = String;

    fn try_from(value: RuntimeValue) -> Result<Self, Self::Error> {
        return match value {
            RuntimeValue::Bool(b) => Ok(b),
            v => Err(format!("Expected boolean,  found {:?}", v)),
        };
    }
}
impl TryFrom<RuntimeValue> for f64 {
    type Error = String;

    fn try_from(value: RuntimeValue) -> Result<Self, Self::Error> {
        return match value {
            RuntimeValue::Number(num) => Ok(num),
            v => Err(format!("Expected number,  found {:?}", v)),
        };
    }
}
