use colored::{ColoredString, Colorize};

use crate::{binding::error, frontend::parser::values::Expression};
#[derive(Clone, Debug, PartialEq)]
pub enum BoundStatement {
    BoundVariableDeclaration {
        is_final: bool,
        identifier: String,
        value: Box<BoundExpression>,
        position: usize,
    },
    BoundExpression(BoundExpression),
}
impl BoundStatement {
    pub fn get_value_type(&self) -> ValueType {
        match self {
            BoundStatement::BoundVariableDeclaration {
                is_final,
                identifier,
                value,
                position,
            } => ValueType::Void,
            BoundStatement::BoundExpression(bound_expression) => bound_expression.get_value_type(),
        }
    }
}
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
        result_type: ValueType,
        position: usize,
    },
    BoundBlock {
        bounded_statements: Vec<BoundStatement>,
        value_type: ValueType,
        position: usize,
    },
    BoundReturn {
        value: Option<Box<BoundExpression>>,
        value_type: ValueType,
        position: usize,
    },
    BoundVariableAssignment {
        identifier: String,
        value: Box<BoundExpression>,
        value_type: ValueType,
        position: usize,
    },
}
impl BoundExpression {
    pub fn get_value_type(&self) -> ValueType {
        match self {
            BoundExpression::BoundReturn {
                value: _,
                value_type,
                position: _,
            } => *value_type,
            BoundExpression::BoundLiteralExpression {
                value: _,
                value_type,
                position: _,
            } => *value_type,
            BoundExpression::BoundUnaryExpression {
                operator: _,
                operand: _,
                value_type,
                position: _,
            } => *value_type,
            BoundExpression::BoundBinaryExpression {
                left: _,
                operator: _,
                right: _,
                result_type: value_type,
                position: _,
            } => *value_type,
            BoundExpression::BoundBlock {
                bounded_statements: _,
                value_type,
                position: _,
            } => *value_type,
            BoundExpression::BoundVariableAssignment {
                identifier: _,
                value: _,
                value_type,
                position: _,
            } => *value_type,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum BoundBinaryExpressionType {
    Addition,
    Subtraction,
    Multiplication,
    Division,

    LogicalOr,
    LogicalAnd,
    Is,
    IsNot,
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
    LogicalNegation,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueType {
    Number,
    String,
    Bool,
    Null,
    Void,
}
impl ValueType {
    pub fn to_string(&self) -> String {
        format!("System.{:?}", self)
    }
}
