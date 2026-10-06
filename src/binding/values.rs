use colored::{ColoredString, Colorize};

use crate::{binding::error, frontend::parser::values::Expression};

#[derive(Clone, Debug, PartialEq)]
pub enum BoundExpression {
    BoundLiteralExpression {
        value: LiteralValue,
        value_type: ValueType,
        position: usize,
    },
    BoundUnaryExpression {
        operator: BoundUnaryOperatorType,
        operand: Box<BoundExpression>,
        value_type: ValueType,
        position: usize,
    },
    BoundBinaryExpression {
        left: Box<BoundExpression>,
        operator: BoundBinaryExpressionType,
        right: Box<BoundExpression>,
        value_type: ValueType,
        position: usize,
    },
}
impl BoundExpression {
    pub fn get_value_type(&self) -> ValueType {
        match self {
            BoundExpression::BoundLiteralExpression {
                value: _,
                value_type,
                position: _,
            } => value_type.clone(),
            BoundExpression::BoundUnaryExpression {
                operator: _,
                operand: _,
                value_type,
                position: _,
            } => value_type.clone(),
            BoundExpression::BoundBinaryExpression {
                left: _,
                operator: _,
                right: _,
                value_type,
                position: _,
            } => value_type.clone(),
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum BoundBinaryExpressionType {
    Addition,
    Substraction,
    Multiplication,
    Devision,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl LiteralValue {
    pub fn from(expr: Expression) -> LiteralValue {
        match expr {
            Expression::Number(num) => LiteralValue::Number(num),
            Expression::Bool(b) => LiteralValue::Bool(b),
            Expression::String(str) => LiteralValue::String(str),
            Expression::Null => LiteralValue::Null,

            var => error(ColoredString::from(format!(
                "Unable to get literal value from {}",
                format!("{:?}", var).bold()
            ))),
        }
    }
    pub fn get_value_type(&self) -> ValueType {
        match self {
            LiteralValue::String(_) => ValueType::String,
            LiteralValue::Number(_) => ValueType::Number,
            LiteralValue::Bool(_) => ValueType::Bool,
            LiteralValue::Null => ValueType::Null,
        }
    }

    pub fn get_value(&self) -> f64 {
        return match self {
            LiteralValue::Number(num) => *num,
            val => error(ColoredString::from(format!(
                "Unable to get value {}",
                format!("{:?}", val).bold()
            ))),
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoundUnaryOperatorType {
    Identity,
    Negation,
}
#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    Number,
    String,
    Bool,
    Null,
}
