use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
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
            variables: HashMap::new(),
            finals: HashSet::new(),
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
    pub fn expect_value_type<T>(self) -> T
    where
        T: TryFrom<RuntimeValue>,
        <T as TryFrom<RuntimeValue>>::Error: Display,
    {
        T::try_from(self).unwrap_or_else(|err| {
            error(ColoredString::from(format!("{}", err)));
        })
    }
    fn get_value_type(&self) -> ValueType {
        match self {
            RuntimeValue::Bool(_) => ValueType::Bool,
            RuntimeValue::String(_) => ValueType::String,
            RuntimeValue::Number(_) => ValueType::Number,
            RuntimeValue::Null => ValueType::Null,
        }
    }
    pub fn to_string(self) -> String {
        match self {
            RuntimeValue::Number(val) => val.to_string(),
            RuntimeValue::String(val) => val.to_string(),
            RuntimeValue::Null => String::from("null"),
            RuntimeValue::Bool(val) => val.to_string(),
        }
    }
    pub fn is_bool(&self) -> bool {
        return self.get_value_type().eq(&ValueType::Bool);
    }

    pub fn is_number(&self) -> bool {
        return self.get_value_type().eq(&ValueType::Number);
    }

    pub fn is_string(&self) -> bool {
        return self.get_value_type().eq(&ValueType::String);
    }

    pub fn is_null(&self) -> bool {
        return self.get_value_type().eq(&ValueType::Null);
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
            result_type: value_type,
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
    position: usize,
    block: &mut Block,
) -> RuntimeValue {
    let left = evaluate_expression(left, block);
    let right = evaluate_expression(right, block);

    if left.is_number() && right.is_number() {
        return evaluate_number_binary_expression(left, right, operator, position);
    } else if left.is_bool() && right.is_bool() {
        return evaluate_bool_binary_expression(left, right, operator, position);
    }
    error(ColoredString::from(format!(
        "Binary expression in not implemented for {} {} {} at position {}",
        left.to_string().bold(),
        format!("{:?}", operator).bold().yellow(),
        right.to_string().bold(),
        position.to_string().bold().blue()
    )));
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
