use crate::binding::bind_binary_expression::bind_binary_expression;
use crate::binding::bind_unary_expression::bind_unary_expression;
use crate::binding::values::{BoundStatement, LiteralValue, ValueType};
use crate::binding::{error, values::BoundExpression};
use crate::frontend::parser::values::{Expression, Statement};
use colored::{ColoredString, Colorize};

pub fn bind_statement(statement: Statement) -> Option<BoundStatement> {
    match statement {
        Statement::VariableDeclaration {
            is_final,
            identifier,
            value,
            position,
        } => Option::Some(bind_variable_declaration(
            is_final, identifier, value, position,
        )),
        Statement::Empty => Option::None,
        Statement::Expression(expression) => {
            Option::Some(BoundStatement::BoundExpression(bind_expression(expression)))
        }
        Statement::Return { value, position } => Option::Some(bind_return(value, position)),
    }
}
pub fn bind_expression(expr: Expression) -> BoundExpression {
    match expr {
        Expression::BinaryExpression {
            left,
            operator,
            right,
            position,
        } => bind_binary_expression(left, operator, right, position),
        Expression::UnaryExpression {
            operator,
            operand,
            position,
        } => bind_unary_expression(operator, operand, position),
        Expression::LiteralExpression { value, position } => {
            bind_literal_expression(*value, position)
        }
        Expression::Block(statements, pos) => bind_block(statements, pos),
        Expression::VariableAssignment {
            identifier,
            value,
            position,
        } => bind_variable_assignment(identifier, value, position),
        _ => error(ColoredString::from(format!(
            "Unexpected expression {} for type binding",
            expr.to_string().bold()
        ))),
    }
}

fn bind_variable_assignment(
    identifier: String,
    value: Box<Expression>,
    position: usize,
) -> BoundExpression {
    let bound_value = bind_expression(*value);
    BoundExpression::BoundVariableAssignment {
        identifier,
        value: Box::new(bound_value.clone()),
        value_type: bound_value.get_value_type(),
        position,
    }
}

fn bind_return(value: Option<Box<Expression>>, position: usize) -> BoundStatement {
    let mut bound_expr = Option::None;
    let mut return_type = ValueType::Void;
    if let Some(val) = value {
        bound_expr = Option::Some(Box::from(bind_expression(*val)));
        return_type = bound_expr.clone().unwrap().get_value_type();
    }
    return BoundStatement::BoundReturn {
        value: bound_expr,
        value_type: return_type,
        position,
    };
}

fn bind_block(statements: Vec<Statement>, position: usize) -> BoundExpression {
    let mut bounded_statements = vec![];

    statements.iter().for_each(|expr| {
        bounded_statements.push(bind_statement(expr.to_owned()).unwrap());
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
    is_final: bool,
    identifier: String,
    value: Expression,
    position: usize,
) -> BoundStatement {
    let bound_value = bind_expression(value);
    return BoundStatement::BoundVariableDeclaration {
        is_final,
        identifier,
        value: Box::new(bound_value),
        position,
    };
}

fn bind_literal_expression(val: Expression, pos: usize) -> BoundExpression {
    let literal = LiteralValue::from(val);
    return BoundExpression::BoundLiteralExpression {
        value: literal.clone(),
        value_type: literal.get_value_type(),
        position: pos,
    };
}
