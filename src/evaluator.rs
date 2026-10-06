use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

use colored::{ColoredString, Colorize};

use crate::binding::values::{
    BoundBinaryExpressionType, BoundExpression, BoundUnaryOperatorType, LiteralValue, ValueType,
};

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<BoundExpression>,
}

impl Program {
    pub fn new(body: Vec<BoundExpression>) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    variables: HashMap<String, RuntimeValue>,
    finals: HashSet<String>,

    parent_block: Option<Box<Block>>,
}

impl Block {
    pub fn new(parent_block: Block) -> Self {
        Self {
            variables: HashMap::new(),
            finals: HashSet::new(),
            parent_block: Option::from(Box::new(parent_block)),
        }
    }
    pub fn program_block() -> Self {
        Self {
            variables: HashMap::from([
                (String::from("true"), RuntimeValue::Bool(true)),
                (String::from("false"), RuntimeValue::Bool(false)),
                (String::from("null"), RuntimeValue::Null),
            ]),
            finals: HashSet::from([
                String::from("true"),
                String::from("false"),
                String::from("null"),
            ]),
            parent_block: Option::None,
        }
    }

    fn insert_variable(
        &mut self,
        is_final: bool,
        identifier: &str,
        value: RuntimeValue,
    ) -> Result<RuntimeValue, ColoredString> {
        if self.is_variable_final(identifier) {
            return Result::Err(ColoredString::from(format!(
                "Invalid operation. Cannot reassign final variable`s '{}' value",
                identifier.bold(),
            )));
        }
        self.variables.insert(identifier.to_string(), value.clone());
        if is_final {
            self.finals.insert(identifier.to_string());
        }
        Result::Ok(value)
    }

    fn is_variable_final(&self, identifier: &str) -> bool {
        if self.finals.contains(identifier) {
            return true;
        }
        if let Some(parent) = &self.parent_block {
            return parent.is_variable_final(identifier);
        }
        false
    }

    fn inspect_variable(&self, identifier: &str) -> Option<&RuntimeValue> {
        if let Some(val) = self.variables.get(identifier) {
            return Option::from(val);
        }

        if let Some(parent) = &self.parent_block {
            return parent.inspect_variable(identifier);
        }

        Option::None

        // error(ColoredString::from(format!(
        //     "Variable {} was not found in the block.",
        //     identifier.bold(),
        // )))
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeValue {
    Number(f64),
    String(String),
    Null,
    Bool(bool),
}
impl RuntimeValue {
    fn get_number_value(&self, position: usize) -> f64 {
        match self {
            RuntimeValue::Number(num) => *num,
            v => error(ColoredString::from(format!(
                "Unable to get the number value at position {}, provided {}",
                position.to_string().bold().blue(),
                format!("{:?}", v.get_value_type()).bold()
            ))),
        }
    }
    fn get_value_type(&self) -> ValueType {
        match self {
            RuntimeValue::Bool(_) => ValueType::Bool,
            RuntimeValue::String(_) => ValueType::String,
            RuntimeValue::Number(_) => ValueType::Number,
            RuntimeValue::Null => ValueType::Null,
        }
    }
}
pub fn evaluate_program(program: Program) {
    let mut program_block = Block::program_block();
    for expression in program.body {
        println!("{:?}", evaluate_expression(expression, &mut program_block));
    }
}

fn evaluate_expression(expression: BoundExpression, block: &mut Block) -> RuntimeValue {
    return match expression {
        BoundExpression::BoundLiteralExpression {
            value,
            value_type,
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
        BoundExpression::BoundUnaryExpression {
            operator,
            operand,
            value_type,
            position,
        } => evaluate_unary_expression((operator, operand), block, position),

        BoundExpression::BoundBinaryExpression {
            left,
            operator,
            right,
            value_type,
            position,
        } => evaluate_binary_expression(*left, operator, *right, position, block),
        _val => error(ColoredString::from(format!(
            "{} is not implemented yet",
            format!("{_val:?}").bold().blue()
        ))),
    };
}

fn evaluate_unary_expression(
    unary_expression: (BoundUnaryOperatorType, Box<BoundExpression>),
    block: &mut Block,
    _position: usize,
) -> RuntimeValue {
    match unary_expression.0 {
        BoundUnaryOperatorType::Identity => evaluate_expression(*unary_expression.1, block),
        BoundUnaryOperatorType::Negation => RuntimeValue::Number(
            -evaluate_expression(*unary_expression.1, block).get_number_value(_position),
        ),
    }
}

fn evaluate_binary_expression(
    left: BoundExpression,
    operator: BoundBinaryExpressionType,
    right: BoundExpression,
    _position: usize,
    block: &mut Block,
) -> RuntimeValue {
    let left: f64 = {
        let value = evaluate_expression(left, block);
        match value {
            RuntimeValue::Number(num) => num,
            _ => error(ColoredString::from(format!(
                "Binary expression in not implemented for {} at position {}",
                format!("{:?}", value.get_value_type()).bold(),
                _position.to_string().bold().blue()
            ))),
        }
    };
    let right: f64 = {
        let value = evaluate_expression(right, block);
        match value {
            RuntimeValue::Number(num) => num,
            _ => error(ColoredString::from(format!(
                "Binary expression in not implemented for {} at position {}",
                format!("{:?}", value.get_value_type()).bold(),
                _position.to_string().bold().blue()
            ))),
        }
    };
    return RuntimeValue::Number(match operator {
        BoundBinaryExpressionType::Addition => left + right,
        BoundBinaryExpressionType::Substraction => left - right,
        BoundBinaryExpressionType::Multiplication => left * right,
        BoundBinaryExpressionType::Devision => left / right,
    });
}

pub fn error(message: ColoredString) -> ! {
    println!(
        " \n =====  \n {} \n {message} \n ===== ",
        "Evaluation Error".red().bold(),
    );
    exit(-1);
}
