use colored::{ColoredString, Colorize};

use crate::{binding::error, frontend::parser::values::Expression};
#[derive(Clone, Debug, PartialEq)]
pub enum BoundStatement {
    BoundVariableDeclaration {
        is_final: bool,
        identifier: String,
        value: Box<BoundExpression>,
        value_type: ValueType,
        position: usize,
    },
    BoundExpression(BoundExpression),
    BoundReturn {
        value: Option<Box<BoundExpression>>,
        value_type: ValueType,
        position: usize,
    },
}
impl BoundStatement {
    pub fn get_value_type(&self) -> ValueType {
        match self {
            BoundStatement::BoundVariableDeclaration {
                is_final: _,
                identifier: _,
                value: _,
                value_type: _,
                position: _,
            } => ValueType::Void,
            BoundStatement::BoundReturn {
                value: _,
                value_type,
                position: _,
            } => *value_type,
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
    pub fn to_string(&self) -> String {
        return format!("{self:?}");
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
    Integer(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl LiteralValue {
    pub fn from(expr: Expression) -> LiteralValue {
        match expr {
            Expression::Integer(num) => LiteralValue::Integer(num),
            Expression::Float(num) => LiteralValue::Float(num),
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
            LiteralValue::Float(_) => ValueType::Float,
            LiteralValue::Integer(_) => ValueType::Integer,
            LiteralValue::Bool(_) => ValueType::Bool,
            LiteralValue::Null => ValueType::Null,
        }
    }

    pub fn get_value(&self) -> NumericValue {
        return match self {
            LiteralValue::Integer(num) => NumericValue::Integer(*num),
            LiteralValue::Float(num) => NumericValue::Float(*num),
            val => error(ColoredString::from(format!(
                "Unable to get value {}",
                format!("{:?}", val).bold()
            ))),
        };
    }
}

pub enum NumericValue {
    Integer(i64),
    Float(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoundUnaryOperatorType {
    Identity,
    Negation,
    LogicalNegation,
}
impl BoundUnaryOperatorType {
    pub fn to_string(&self) -> String {
        return format!("{:?}", self);
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ValueType {
    Float,
    Integer,
    String,
    Bool,
    Null,
    Void,
}
impl ValueType {
    pub fn from_string(val: &str) -> Self {
        match val {
            "Float" => ValueType::Float,
            "Integer" => ValueType::Integer,
            "String" => ValueType::String,
            "Bool" => ValueType::Bool,
            "Null" => ValueType::Null,
            def => error(ColoredString::from(format!("Unsupported value type {def}"))),
        }
    }
    pub fn to_string(&self) -> String {
        format!("System.{:?}", self)
    }
    pub fn is_numeric(&self) -> bool {
        match self {
            ValueType::Float | ValueType::Integer => true,
            _ => false,
        }
    }
    pub fn is_bool(&self) -> bool {
        match self {
            ValueType::Bool => true,
            _ => false,
        }
    }
    pub fn is_null(&self) -> bool {
        match self {
            ValueType::Null => true,
            _ => false,
        }
    }
}
