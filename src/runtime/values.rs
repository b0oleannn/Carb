use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::Display,
    rc::Rc,
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
    variables: HashMap<String, Variable>,
    finals: HashSet<String>,

    parent_block: Option<Rc<RefCell<Block>>>,
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub value_type: ValueType,
    pub value: RuntimeValue,
}

impl Variable {
    pub fn new(value_type: ValueType, value: RuntimeValue) -> Self {
        Self { value_type, value }
    }
}
impl Block {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            finals: HashSet::new(),
            parent_block: Option::None,
        }
    }
    pub fn new_child(parent: Rc<RefCell<Block>>) -> Self {
        Self {
            variables: HashMap::new(),
            finals: HashSet::new(),
            parent_block: Option::Some(parent),
        }
    }
    pub fn declare_variable(
        &mut self,
        is_final: bool,
        identifier: &str,
        value: RuntimeValue,
        value_type: ValueType,
    ) -> Result<RuntimeValue, ColoredString> {
        if self.variables.contains_key(identifier) {
            return Result::Err(ColoredString::from(format!(
                "Unable to declare variable in the block. The variable '{}' has been already declared ",
                identifier.bold(),
            )));
        }
        self.variables.insert(
            identifier.to_string(),
            Variable::new(value_type, value.clone()),
        );
        if is_final {
            self.finals.insert(identifier.to_string());
        }
        Result::Ok(value)
    }
    pub fn assign_variable(
        &mut self,
        identifier: &str,
        value: &RuntimeValue,
    ) -> Result<RuntimeValue, ColoredString> {
        if let Some(prev_var) = self.inspect_variable(identifier) {
            if self.is_variable_final(identifier) {
                return Result::Err(ColoredString::from(format!(
                    "Unable to reassign the the variable`s '{}' value. The variable is final",
                    identifier.red().bold(),
                )));
            }
            self.variables.insert(
                identifier.to_string(),
                Variable::new(prev_var.value_type, value.clone()),
            );
            return Result::Ok(value.clone());
        }
        return Result::Err(ColoredString::from(format!(
            "Unable to reassign the the variable`s '{}' value. The variable wasn`t found in the block",
            identifier.red().bold(),
        )));
    }

    fn is_variable_final(&self, identifier: &str) -> bool {
        if self.finals.contains(identifier) {
            return true;
        }
        if let Some(parent) = &self.parent_block {
            return parent.borrow().is_variable_final(identifier);
        }
        false
    }

    pub fn inspect_variable(&self, identifier: &str) -> Option<Variable> {
        if let Some(val) = self.variables.get(identifier) {
            return Option::from(val.clone());
        }

        if let Some(parent) = &self.parent_block {
            return parent.borrow().inspect_variable(identifier);
        }

        Option::None
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
