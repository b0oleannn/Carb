#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Expression>,
}

impl Program {
    pub fn new(body: Vec<Expression>) -> Self {
        Self { body }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64, usize),
    String(String, usize),
    Bool(bool, usize),
    Null(usize),

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
