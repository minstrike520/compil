pub enum Statement {
    Program(Program),
    NumericLiteral(NumericLiteral),
    Identifier(Identifier),
    BinaryExpression(BinaryExpression),
}

pub enum Expression {
    BinaryExpression(BinaryExpression),
    Identifier(Identifier)
}

pub struct Program { body: Vec<Statement> }

pub struct NumericLiteral { value: i32 }

pub struct Identifier { symbol: String }

pub struct BinaryExpression {
    left: Option<Box<Expression>>,
    right: Option<Box<Expression>>,
    operator: String
}
