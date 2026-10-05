use std::{
    collections::{HashMap, HashSet},
    process::exit,
};

use colored::{ColoredString, Colorize};

use crate::frontend::parser::values::{Expression, Program};

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
pub fn evaluate_program(program: Program) {
    let mut program_block = Block::program_block();
    for expression in program.body {
        println!("{:?}", evaluate_expression(expression, &mut program_block));
    }
}

fn evaluate_expression(expression: Expression, block: &mut Block) -> RuntimeValue {
    return match expression {
        Expression::Number(num, _pos) => RuntimeValue::Number(num),
        Expression::Bool(b, _pos) => RuntimeValue::Bool(b),
        Expression::String(str, _pos) => RuntimeValue::String(str),
        Expression::Null(_pos) => RuntimeValue::Null,
        Expression::Identifier(identifier, pos) => {
            evaluate_identifier(identifier, pos, block).clone()
        }

        Expression::VariableDeclaration {
            is_final,
            identifier,
            value,
            position,
        } => evaluate_variable_declaration(
            _VariableDeclaration::new(is_final, identifier, *value, position),
            block,
        ),

        Expression::BinaryExpression {
            left,
            operator,
            right,
            position,
        } => evaluate_binary_expression(
            _BinaryExpression::new(*left, operator, *right, position),
            block,
        ),

        Expression::Block(expressions, position) => evaluate_block(expressions, position, block),

        _val => error(ColoredString::from(format!(
            "{} is not implemented yet",
            format!("{_val:?}").bold().blue()
        ))),
    };
}

fn evaluate_binary_expression(
    binary_expression: _BinaryExpression,
    block: &mut Block,
) -> RuntimeValue {
    let left: f64 = {
        let runtime_value = evaluate_expression(binary_expression.left, block);
        match runtime_value {
            RuntimeValue::Number(num) => num,

            _ => error(ColoredString::from(format!(
                "{} is implemented only for numbers",
                "BinaryExpression".bold()
            ))),
        }
    };
    let right: f64 = {
        let runtime_value = evaluate_expression(binary_expression.right, block);
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
            binary_expression.position.to_string().blue()
        ))),
    };
}

fn evaluate_variable_declaration(
    variable_declaration: _VariableDeclaration,
    block: &mut Block,
) -> RuntimeValue {
    let val = evaluate_expression(variable_declaration.value, block);
    match block.insert_variable(
        variable_declaration.is_final,
        &variable_declaration.identifier,
        val,
    ) {
        Ok(value) => value.clone(),
        Err(message) => error(ColoredString::from(format!(
            "{} at position {}",
            message,
            variable_declaration.position.to_string().bold().blue()
        ))),
    }
}

fn evaluate_identifier(identifier: String, position: usize, block: &mut Block) -> &RuntimeValue {
    return match block.inspect_variable(&identifier) {
        Some(val) => val,
        None => error(ColoredString::from(format!(
            "Variable '{}' wasn`t found in the block at position {}",
            identifier.bold(),
            position.to_string().bold().blue()
        ))),
    };
}

fn evaluate_block(
    expressions: Vec<Expression>,
    position: usize,
    parent_block: &mut Block,
) -> RuntimeValue {
    let mut child_block = Block::new(parent_block.clone());
    let mut last = RuntimeValue::Null;
    for expression in expressions {
        last = evaluate_expression(expression, &mut child_block);
    }
    last
}

struct _VariableDeclaration {
    is_final: bool,
    identifier: String,
    value: Expression,
    position: usize,
}
impl _VariableDeclaration {
    fn new(is_final: bool, identifier: String, value: Expression, position: usize) -> Self {
        Self {
            is_final,
            identifier,
            value,
            position,
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
