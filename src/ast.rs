
#[derive(Debug)]
pub enum Statement {
    Program(Program),
    BinaryExpression(BinaryExpression),
    Expression(Expression)
}

#[derive(Debug)]
pub enum Expression {
    NumericLiteral(NumericLiteral),
    Identifier(Identifier)
}
#[derive(Debug)]
pub struct Program { pub body: Vec<Statement> }

impl Program {
    pub fn new() -> Self { Self { body: Vec::new() } }
}

#[derive(Debug)]
pub struct NumericLiteral { pub value: i32 }

impl NumericLiteral {
    pub fn create(value: i32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub struct Identifier { pub symbol: String }

impl Identifier {
    pub fn create(symbol: String) -> Self {
        Self { symbol }
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    left: Option<Box<Expression>>,
    right: Option<Box<Expression>>,
    operator: String
}
