use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

use colored::{ColoredString, Colorize};

use crate::frontend::parser::values::{Expression, Program};

pub struct Evaluator {
    block: Block,
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

    fn insert_variable(&mut self, is_final: bool, identifier: &str, value: RuntimeValue) {
        if self.is_variable_final(identifier) {
            error(ColoredString::from(format!(
                "Invalid operation. Cannot reassign final variable`s '{}' value",
                identifier.bold(),
            )))
        }
        self.variables.insert(identifier.to_string(), value);
        if is_final {
            self.finals.insert(identifier.to_string());
        }
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
impl Evaluator {
    pub fn evaluate_program(&mut self, program: Program) {
        for expression in program.body {
            println!("{:?}", self.evaluate_expression(expression));
        }
    }

    fn evaluate_expression(&mut self, expression: Expression) -> RuntimeValue {
        return match expression {
            Expression::Number(num, _pos) => RuntimeValue::Number(num),
            Expression::Bool(b, _pos) => RuntimeValue::Bool(b),
            Expression::String(str, _pos) => RuntimeValue::String(str),
            Expression::Null(_pos) => RuntimeValue::Null,
            Expression::Identifier(identifier, _pos) => {
                self.evaluate_identifier(identifier).clone()
            }

            Expression::VariableDeclaration {
                is_final,
                identifier,
                value,
            } => self.evaluate_variable_declaration(_VariableDeclaration::new(
                is_final, identifier, *value,
            )),

            Expression::BinaryExpression {
                left,
                operator,
                right,
                position,
            } => self.evaluate_binary_expression(_BinaryExpression::new(
                *left, operator, *right, position,
            )),

            val => error(ColoredString::from(format!(
                "{} is not implemented yet",
                format!("{val:?}").bold()
            ))),
        };
    }

    fn evaluate_binary_expression(&mut self, binary_expression: _BinaryExpression) -> RuntimeValue {
        let left: f64 = {
            let runtime_value = self.evaluate_expression(binary_expression.left);
            match runtime_value {
                RuntimeValue::Number(num) => num,

                _ => error(ColoredString::from(format!(
                    "{} is implemented only for numbers",
                    "BinaryExpression".bold()
                ))),
            }
        };
        let right: f64 = {
            let runtime_value = self.evaluate_expression(binary_expression.right);
            match runtime_value {
                RuntimeValue::Number(num) => num,

                _ => error(ColoredString::from(format!(
                    "{} is implemented only for numbers",
                    "BinaryExpression".bold()
                ))),
            }
        };
        return match binary_expression.operator.as_str() {
            "+" => RuntimeValue::Number(left + right),
            "-" => RuntimeValue::Number(left - right),

            "/" => RuntimeValue::Number(left / right),
            "*" => RuntimeValue::Number(left * right),
            val => error(ColoredString::from(format!(
                "Unsuported operation {} at position {}",
                val.bold(),
                binary_expression.position
            ))),
        };
    }

    fn evaluate_variable_declaration(
        &mut self,
        variable_declaration: _VariableDeclaration,
    ) -> RuntimeValue {
        let value = self.evaluate_expression(variable_declaration.value);
        self.block.insert_variable(
            variable_declaration.is_final,
            &variable_declaration.identifier,
            value.clone(),
        );
        value
    }

    fn evaluate_identifier(&self, identifier: String) -> &RuntimeValue {
        return match self.block.inspect_variable(&identifier) {
            Some(val) => val,
            None => error(ColoredString::from(format!(
                "Variable '{}' wasn`t found in the block",
                identifier.bold(),
            ))),
        };
    }

    pub fn new() -> Self {
        Self {
            block: Block::program_block(),
        }
    }
}
struct _VariableDeclaration {
    is_final: bool,
    identifier: String,
    value: Expression,
}
impl _VariableDeclaration {
    fn new(is_final: bool, identifier: String, value: Expression) -> Self {
        Self {
            is_final,
            identifier,
            value,
        }
    }
}

struct _BinaryExpression {
    left: Expression,
    operator: String,
    right: Expression,
    position: usize,
}
impl _BinaryExpression {
    fn new(left: Expression, operator: String, right: Expression, position: usize) -> Self {
        Self {
            left,
            operator,
            right,
            position,
        }
    }
}

pub fn error(message: ColoredString) -> ! {
    println!(
        " \n =====  \n {} \n {message} \n ===== ",
        "Evaluation Error".red().bold(),
    );
    exit(-1);
}
