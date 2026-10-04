#[derive(Debug)]
pub(crate) enum Statement {
    Expr(Expression),
}

#[derive(Debug)]
pub(crate) enum Expression {
    Number(f64),

    BinaryExpression {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}
