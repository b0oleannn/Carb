use std::{cell::RefCell, process::exit, rc::Rc};

use colored::{ColoredString, Colorize};

use crate::{
    binding::values::{
        BoundBinaryExpressionType, BoundExpression, BoundStatement, BoundUnaryOperatorType,
        LiteralValue, ValueType,
    },
    runtime::values::{Block, Program, RuntimeValue},
};

pub fn evaluate_program(program: Program) {
    let program_block = Rc::from(RefCell::from(Block::new()));
    for statement in program.body {
        println!("{:?}", evaluate_statement(statement, &program_block));
    }
}
fn evaluate_statement(
    statement: BoundStatement,
    block: &Rc<RefCell<Block>>,
) -> Option<RuntimeValue> {
    match statement {
        BoundStatement::BoundVariableDeclaration {
            is_final,
            identifier,
            value,
            value_type,
            position,
        } => {
            evaluate_variable_declaration(is_final, identifier, value, value_type, position, block);
            Option::None
        }
        BoundStatement::BoundExpression(bound_expression) => {
            Option::from(evaluate_expression(bound_expression, block))
        }
        BoundStatement::BoundReturn {
            value,
            value_type,
            position,
        } => Option::from(evaluate_return_statement(value, block, position)),
    }
}

fn evaluate_return_statement(
    value: Option<Box<BoundExpression>>,
    block: &Rc<RefCell<Block>>,
    position: usize,
) -> RuntimeValue {
    if let Some(v) = value {
        return evaluate_expression(*v, block);
    }
    return RuntimeValue::Null;
}

fn evaluate_variable_declaration(
    is_final: bool,
    identifier: String,
    value: Box<BoundExpression>,
    value_type: ValueType,
    position: usize,
    block: &Rc<RefCell<Block>>,
) {
    let val = evaluate_expression(*value, block);
    match block
        .borrow_mut()
        .declare_variable(is_final, &identifier, val, value_type)
    {
        Ok(v) => {
            println!("Declared variable {identifier}: {v:?}")
        }
        Err(msg) => error(ColoredString::from(format!(
            "{msg} at position `{}`",
            position.to_string().bold().blue(),
        ))),
    }
}
fn evaluate_expression(expression: BoundExpression, block: &Rc<RefCell<Block>>) -> RuntimeValue {
    return match expression {
        BoundExpression::BoundLiteralExpression {
            value,
            value_type: _,
            position,
        } => {
            return match value {
                LiteralValue::Integer(int) => RuntimeValue::Integer(int),
                LiteralValue::Float(float) => RuntimeValue::Float(float),
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
        BoundExpression::BoundBlock {
            bounded_statements,
            value_type,
            position,
        } => evaluate_block(bounded_statements, value_type, position, block),

        _val => error(ColoredString::from(format!(
            "{} is not implemented yet",
            format!("{_val:?}").bold().blue()
        ))),
    };
}

fn evaluate_block(
    bounded_statements: Vec<BoundStatement>,
    _value_type: ValueType,
    _position: usize,
    parent_block: &Rc<RefCell<Block>>,
) -> RuntimeValue {
    let current_block = Rc::new(RefCell::new(Block::new_child(Rc::clone(parent_block))));
    for statement in bounded_statements.iter() {
        match statement {
            BoundStatement::BoundReturn {
                value: _,
                value_type: _,
                position: _,
            } => {
                return evaluate_statement(statement.to_owned(), &current_block).unwrap();
            }
            _ => {
                evaluate_statement(statement.to_owned(), &current_block);
            }
        }
    }
    return RuntimeValue::Null;
}

fn evaluate_variable_assignment(
    identifier: String,
    value: Box<BoundExpression>,
    value_type: ValueType,
    position: usize,
    block: &Rc<RefCell<Block>>,
) -> RuntimeValue {
    let val = evaluate_expression(*value, block);
    let mut block = block.borrow_mut();
    if let Some(stored_variable) = block.inspect_variable(&identifier) {
        if stored_variable.value_type.eq(&ValueType::Null) {
            println!("{stored_variable:?}");
            match block.assign_variable(&identifier, &val) {
                Ok(run) => return run,
                Err(err) => {
                    error(ColoredString::from(format!(
                        "{err} at position {}",
                        position.to_string().bold().blue()
                    )));
                }
            }
        } else if value_type.eq(&ValueType::Null) || stored_variable.value_type.eq(&value_type) {
            println!("{stored_variable:?}");
            match block.assign_variable(&identifier, &val) {
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
                stored_variable.value.clone().to_string().bold().white(),
                stored_variable
                    .value_type
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
    block: &Rc<RefCell<Block>>,
    position: usize,
) -> RuntimeValue {
    return match unary_expression.1.get_value_type().clone() {
        ValueType::Integer => match unary_expression.0 {
            BoundUnaryOperatorType::Identity => evaluate_expression(*unary_expression.1, block),
            BoundUnaryOperatorType::Negation => RuntimeValue::Integer(
                -evaluate_expression(*unary_expression.1, block).expect_value_type::<i64>(),
            ),
            operation => {
                error(ColoredString::from(format!(
                    "Unary expression in not implemented for {} {} at position {}",
                    operation.to_string().bold(),
                    unary_expression.1.to_string().bold(),
                    position.to_string().bold().blue(),
                )));
            }
        },
        ValueType::Float => match unary_expression.0 {
            BoundUnaryOperatorType::Identity => evaluate_expression(*unary_expression.1, block),
            BoundUnaryOperatorType::Negation => RuntimeValue::Float(
                -evaluate_expression(*unary_expression.1, block).expect_value_type::<f64>(),
            ),
            operation => {
                error(ColoredString::from(format!(
                    "Unary expression in not implemented for {} {} at position {}",
                    operation.to_string().bold(),
                    unary_expression.1.to_string().bold(),
                    position.to_string().bold().blue(),
                )));
            }
        },
        ValueType::Bool => match unary_expression.0 {
            BoundUnaryOperatorType::LogicalNegation => RuntimeValue::Bool(
                !evaluate_expression(*unary_expression.1, block).expect_value_type::<bool>(),
            ),
            operation => {
                error(ColoredString::from(format!(
                    "Unary expression in not implemented for {} {} at position {}",
                    operation.to_string().bold(),
                    unary_expression.1.to_string().bold(),
                    position.to_string().bold().blue(),
                )));
            }
        },
        _ => exit(-1),
    };
}

fn evaluate_binary_expression(
    left: BoundExpression,
    operator: BoundBinaryExpressionType,
    right: BoundExpression,
    result_type: ValueType,
    position: usize,
    block: &Rc<RefCell<Block>>,
) -> RuntimeValue {
    let left = evaluate_expression(left, block);
    let right = evaluate_expression(right, block);

    match result_type {
        ValueType::Integer | ValueType::Float => {
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
    match (left.get_value_type(), right.get_value_type()) {
        (ValueType::Float, ValueType::Float) => {
            let left = left.expect_value_type::<f64>();
            let right = right.expect_value_type::<f64>();
            return match operator {
                BoundBinaryExpressionType::Addition => RuntimeValue::Float(left + right),
                BoundBinaryExpressionType::Subtraction => RuntimeValue::Float(left - right),
                BoundBinaryExpressionType::Multiplication => RuntimeValue::Float(left * right),
                BoundBinaryExpressionType::Division => RuntimeValue::Float(left / right),
                BoundBinaryExpressionType::Is => RuntimeValue::Bool(left == right),
                BoundBinaryExpressionType::IsNot => RuntimeValue::Bool(left != right),
                v => error(ColoredString::from(format!(
                    "Binary expression in not implemented between {} {} {} at position {}",
                    left.to_string().bold(),
                    format!("{:?}", v).bold().yellow(),
                    right.to_string().bold(),
                    position.to_string().bold().blue()
                ))),
            };
        }

        (ValueType::Integer, ValueType::Integer) => {
            let left = left.expect_value_type::<i64>();
            let right = right.expect_value_type::<i64>();
            return match operator {
                BoundBinaryExpressionType::Addition => RuntimeValue::Integer(left + right),
                BoundBinaryExpressionType::Subtraction => RuntimeValue::Integer(left - right),
                BoundBinaryExpressionType::Multiplication => RuntimeValue::Integer(left * right),
                BoundBinaryExpressionType::Division => RuntimeValue::Integer(left / right),
                BoundBinaryExpressionType::Is => RuntimeValue::Bool(left == right),
                BoundBinaryExpressionType::IsNot => RuntimeValue::Bool(left != right),
                v => error(ColoredString::from(format!(
                    "Binary expression in not implemented between {} {} {} at position {}",
                    left.to_string().bold(),
                    format!("{:?}", v).bold().yellow(),
                    right.to_string().bold(),
                    position.to_string().bold().blue()
                ))),
            };
        }
        (l, r) => error(ColoredString::from(format!(
            "Binary expression in not implemented between {} {} {} at position {}",
            l.to_string().bold(),
            format!("{:?}", operator).bold().yellow(),
            r.to_string().bold(),
            position.to_string().bold().blue()
        ))),
    }
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
            RuntimeValue::Float(num) => Ok(num),
            v => Err(format!("Expected number,  found {:?}", v)),
        };
    }
}
impl TryFrom<RuntimeValue> for i64 {
    type Error = String;

    fn try_from(value: RuntimeValue) -> Result<Self, Self::Error> {
        return match value {
            RuntimeValue::Integer(num) => Ok(num),
            v => Err(format!("Expected number,  found {:?}", v)),
        };
    }
}
