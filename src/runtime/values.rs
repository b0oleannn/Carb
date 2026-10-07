use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use colored::{ColoredString, Colorize};

use crate::{
    binding::values::{BoundStatement, ValueType},
    runtime::evaluator::error,
};

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<BoundStatement>,
}

impl Program {
    pub fn new(body: Vec<BoundStatement>) -> Self {
        Self { body }
    }
}

#[derive(Debug)]
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

    pub fn declare_variable(
        &mut self,
        is_final: bool,
        identifier: &str,
        value: RuntimeValue,
    ) -> Result<RuntimeValue, ColoredString> {
        if self.variables.contains_key(identifier) {
            return Result::Err(ColoredString::from(format!(
                "Unable to declare variable in the block. The variable '{}' has been already declared ",
                identifier.bold(),
            )));
        }
        self.variables.insert(identifier.to_string(), value.clone());
        if is_final {
            self.finals.insert(identifier.to_string());
        }
        Result::Ok(value)
    }
    pub fn assign_variable(
        &mut self,
        identifier: &str,
        value: RuntimeValue,
    ) -> Result<RuntimeValue, ColoredString> {
        if self.is_variable_final(identifier) {
            return Result::Err(ColoredString::from(format!(
                "Unable to reassign the the variable`s '{}' value. The variable is final",
                identifier.red().bold(),
            )));
        }
        self.variables.insert(identifier.to_string(), value.clone());
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

    pub fn inspect_variable(&self, identifier: &str) -> Option<&RuntimeValue> {
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
    Float(f64),
    Integer(i64),
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
    pub fn get_value_type(&self) -> ValueType {
        match self {
            RuntimeValue::Bool(_) => ValueType::Bool,
            RuntimeValue::String(_) => ValueType::String,
            RuntimeValue::Integer(_) => ValueType::Integer,
            RuntimeValue::Float(_) => ValueType::Float,
            RuntimeValue::Null => ValueType::Null,
        }
    }
    pub fn to_string(self) -> String {
        match self {
            RuntimeValue::Integer(val) => val.to_string(),
            RuntimeValue::Float(val) => val.to_string(),
            RuntimeValue::String(val) => val.to_string(),
            RuntimeValue::Null => String::from("null"),
            RuntimeValue::Bool(val) => val.to_string(),
        }
    }
    pub fn is_bool(&self) -> bool {
        return self.get_value_type().eq(&ValueType::Bool);
    }

    pub fn is_float(&self) -> bool {
        return self.get_value_type().eq(&ValueType::Float);
    }

    pub fn is_integer(&self) -> bool {
        return self.get_value_type().eq(&ValueType::Integer);
    }

    pub fn is_string(&self) -> bool {
        return self.get_value_type().eq(&ValueType::String);
    }

    pub fn is_null(&self) -> bool {
        return self.get_value_type().eq(&ValueType::Null);
    }
}
