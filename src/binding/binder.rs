use std::collections::{HashMap, HashSet};

use crate::binding::bind_binary_expression::bind_binary_expression;
use crate::binding::bind_unary_expression::bind_unary_expression;
use crate::binding::values::{BoundStatement, LiteralValue, ValueType};
use crate::binding::{error, values::BoundExpression};
use crate::frontend::parser::values::{Expression, Statement};
use colored::{ColoredString, Colorize};

pub struct Binder {
    //                 depth      identifier  value type
    //                   |              |         |
    variables: HashMap<usize, HashMap<String, ValueType>>,
}

impl Binder {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn bind_statement(&mut self, statement: Statement) -> Option<BoundStatement> {
        match statement {
            Statement::VariableDeclaration {
                depth,
                is_final,
                identifier,
                value,
                declared_type,
                position,
            } => Option::Some(self.bind_variable_declaration(
                depth,
                is_final,
                identifier,
                value,
                declared_type,
                position,
            )),
            Statement::Empty => Option::None,
            Statement::Expression(expression) => Option::Some(BoundStatement::BoundExpression(
                self.bind_expression(expression),
            )),
            Statement::Return { value, position } => {
                Option::Some(self.bind_return(value, position))
            }
        }
    }
    pub fn bind_expression(&mut self, expr: Expression) -> BoundExpression {
        match expr {
            Expression::BinaryExpression {
                left,
                operator,
                right,
                position,
            } => bind_binary_expression(self, left, operator, right, position),
            Expression::UnaryExpression {
                operator,
                operand,
                position,
            } => bind_unary_expression(self, operator, operand, position),
            Expression::LiteralExpression { value, position } => {
                self.bind_literal_expression(*value, position)
            }
            Expression::Block(statements, pos) => self.bind_block(statements, pos),
            Expression::VariableAssignment {
                depth,
                identifier,
                value,
                position,
            } => self.bind_variable_assignment(depth, identifier, value, position),
            Expression::VariableCall {
                depth,
                identifier,
                position,
            } => self.bind_variable_call(depth, identifier, position),
            _ => error(ColoredString::from(format!(
                "Unexpected expression {} for type binding",
                expr.to_string().bold()
            ))),
        }
    }

    fn bind_variable_call(
        &self,
        depth: usize,
        identifier: String,
        position: usize,
    ) -> BoundExpression {
        //  letf a = 10; - 0
        //  {
        //      a + 10; - 1
        //  }
        if let Some(r#type) = self.inspect_variable(depth, &identifier) {
            return BoundExpression::BoundVariableCall {
                identifier,
                value_type: r#type,
                position,
            };
        }
        error(ColoredString::from(format!(
            "Unexpected operation at position {}. The variable {} was not found in the block.",
            position.to_string().bold().blue(),
            identifier.to_string().bold().yellow(),
        )));
    }

    fn bind_variable_assignment(
        &mut self,
        depth: usize,
        identifier: String,
        value: Box<Expression>,
        position: usize,
    ) -> BoundExpression {
        if self.inspect_variable(depth, &identifier).is_some() {
            let bound_value = self.bind_expression(*value);
            return BoundExpression::BoundVariableAssignment {
                identifier,
                value: Box::new(bound_value.clone()),
                value_type: bound_value.get_value_type(),
                position,
            };
        }
        error(ColoredString::from(format!(
            "Unexpected operation at position {}. The variable {} was not found in the block.",
            position.to_string().bold().blue(),
            identifier.to_string().bold().yellow(),
        )));
    }

    fn bind_return(&mut self, value: Option<Box<Expression>>, position: usize) -> BoundStatement {
        let mut bound_expr = Option::None;
        let mut return_type = ValueType::Void;
        if let Some(val) = value {
            bound_expr = Option::Some(Box::from(self.bind_expression(*val)));
            return_type = bound_expr.clone().unwrap().get_value_type();
        }
        return BoundStatement::BoundReturn {
            value: bound_expr,
            value_type: return_type,
            position,
        };
    }

    fn bind_block(&mut self, statements: Vec<Statement>, position: usize) -> BoundExpression {
        let mut bounded_statements = vec![];

        statements.iter().for_each(|expr| {
            bounded_statements.push(self.bind_statement(expr.to_owned()).unwrap());
        });
        return BoundExpression::BoundBlock {
            bounded_statements: bounded_statements.clone(),
            value_type: match bounded_statements.is_empty() {
                true => ValueType::Null,
                false => bounded_statements.last().unwrap().get_value_type(),
            },
            position,
        };
    }

    fn bind_variable_declaration(
        &mut self,
        depth: usize,
        is_final: bool,
        identifier: String,
        value: Expression,
        declared_type: Option<String>,
        position: usize,
    ) -> BoundStatement {
        let bound_value = self.bind_expression(value);
        let value_type;
        match declared_type {
            Some(val) => {
                value_type = ValueType::from_string(&val);
                if value_type.is_null() {
                    error(ColoredString::from(format!(
                        "Invalid operation at position {}. Variable is unable to have null value type",
                        position.to_string().bold().blue()
                    )));
                }
                if !value_type.eq(&bound_value.get_value_type())
                    && !bound_value.get_value_type().is_null()
                {
                    error(ColoredString::from(format!(
                        "Type error at position {}. The declared type {} mismatches with the expression`s type {}",
                        position.to_string().bold().blue(),
                        value_type.to_string().bold().yellow(),
                        bound_value.get_value_type().to_string().bold().yellow(),
                    )));
                }
            }
            None => {
                value_type = if bound_value.get_value_type().is_null() {
                    error(ColoredString::from(format!(
                        "Invalid operation at position {}. Failed to assign value type ",
                        position.to_string().bold().blue()
                    )));
                } else {
                    bound_value.get_value_type()
                }
            }
        }
        if let Some(set) = self.variables.get_mut(&depth) {
            set.insert(identifier.clone(), value_type);
        } else {
            self.variables
                .insert(depth, HashMap::from([(identifier.clone(), value_type)]));
        }
        return BoundStatement::BoundVariableDeclaration {
            is_final,
            identifier,
            value: Box::from(bound_value),
            value_type,
            position,
        };
    }

    fn bind_literal_expression(&self, val: Expression, pos: usize) -> BoundExpression {
        let literal = LiteralValue::from(val);
        return BoundExpression::BoundLiteralExpression {
            value: literal.clone(),
            value_type: literal.get_value_type(),
            position: pos,
        };
    }

    fn inspect_variable(&self, depth: usize, identifier: &str) -> Option<ValueType> {
        if let Some(map) = self.variables.get(&depth) {
            if let Some(t) = map.get(identifier) {
                print!("found variable {} : {:?}", identifier, t);
                return Some(*t);
            }
        }
        if depth == 0 {
            return Option::None;
        }
        self.inspect_variable(depth - 1, identifier)
    }
}
