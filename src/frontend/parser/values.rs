use std::fmt::Alignment::Left;

use colored::{ColoredString, Colorize};

use crate::{binding::values::ValueType, frontend::error};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    String(String),
    Bool(bool),
    Null,

    LiteralExpression {
        value: Box<Expression>,
        position: usize,
    },

    Identifier(String, usize),

    Block(Vec<Expression>, usize),

    BinaryExpression {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
        position: usize,
    },
    UnaryExpression {
        operator: String,
        operand: Box<Expression>,
        position: usize,
    },

    VariableDeclaration {
        is_final: bool,
        identifier: String,
        value: Box<Expression>,
        position: usize,
    },
}
impl Expression {
    pub fn to_string(&self) -> String {
        return format!("{:?}", self);
    }
    pub fn get_type(&self) -> ValueType {
        match self {
            Expression::Bool(_) => ValueType::Bool,
            Expression::Number(_) => ValueType::Number,
            Expression::String(_) => ValueType::String,
            Expression::Null => ValueType::Null,
            Expression::LiteralExpression { value, position: _ } => value.get_type(),
            Expression::UnaryExpression {
                operator: _,
                operand,
                position: _,
            } => operand.get_type(),
            Expression::BinaryExpression {
                left,
                operator: _,
                right: _,
                position: _,
            } => left.get_type(),
            _ => error(ColoredString::from(format!(
                "Failed to get type of expression {}",
                self.to_string().yellow(),
            ))),
        }
    }
}
