#[derive(Debug, Clone)]

pub enum Statement {
    VariableDeclaration {
        is_final: bool,
        identifier: String,
        declared_type: Option<String>,
        value: Expression,
        position: usize,
    },
    Return {
        value: Option<Box<Expression>>,
        position: usize,
    },
    Empty,

    Expression(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,

    LiteralExpression {
        value: Box<Expression>,
        position: usize,
    },

    VariableCall(String, usize),

    Block(Vec<Statement>, usize),

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
    VariableAssignment {
        identifier: String,
        value: Box<Expression>,
        position: usize,
    },
}
impl Expression {
    pub fn to_string(&self) -> String {
        return format!("{:?}", self);
    }
}
