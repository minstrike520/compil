#[derive(Debug)]
pub enum Statement {
    Program(Program),
    Expression(Expression)
}

#[derive(Debug)]
pub enum Expression {
    NumericLiteral(i32),
    Identifier(String),
    NullLiteral,
    BinaryExpression {
        left: Box<Expression>,
        right: Box<Expression>,
        operator: String
    }
}


#[derive(Debug)]
pub struct Program { pub body: Vec<Statement> }

impl Program {
    pub fn new() -> Self { Self { body: Vec::new() } }
}
