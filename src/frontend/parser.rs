use std::collections::VecDeque;

use super::{ast::{Expression, Program, Statement}, lexer::{tokenize, BinaryOperator, Token}};

pub struct Parser {
    tokens: VecDeque<Token>
}

impl Parser {
    pub fn initialize(source_code: String) -> Self {
        Self{ tokens: VecDeque::from(tokenize(source_code)) }
    }
    fn eof(&self) -> bool {
        self.tokens[0] == Token::EOF
    }
    fn at(&self) -> &Token {
        &self.tokens[0]
    }
    /// # Caution
    /// The Token Vec should always be not empty.
    /// Also, the last token should always be **EOF**.
    fn pop_front(&mut self) -> Token {
        self.tokens.pop_front().unwrap()
            }
    fn parse_additive_expression(&mut self) -> Expression {
        let mut left = self.parse_multiplicitave_expression();

        while matches!(self.at(), Token::BinaryOperator(BinaryOperator::Additive(_))) {
            let operator = self.pop_front().to_string();
            let right = self.parse_multiplicitave_expression();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }
        }
        left
    }
    fn parse_multiplicitave_expression(&mut self) -> Expression {
        let mut left = self.parse_primary_expression();
        while matches!(self.at(), Token::BinaryOperator(BinaryOperator::Multiplicitave(_))) {
            let operator = self.pop_front().to_string();
            let right = self.parse_primary_expression();
            left = Expression::BinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }
        }
        left
    }
    fn parse_primary_expression(&mut self) -> Expression {
        let token = self.pop_front();
        match token {
            Token::Identifier(value) => Expression::Identifier(value),
            Token::Number(value) => Expression::NumericLiteral(value.parse::<i32>().unwrap()),
            Token::OpenParen => {
                let expr = self.parse_expression();
                assert!(self.pop_front() == Token::CloseParen,
                    "Expected a close parenthesis"
                );
                expr
            }
            _ => {
                panic!("Unexpected token found during parsing: {}", token.to_string())
            }
        }
    }
    fn parse_expression(&mut self) -> Expression {
        self.parse_additive_expression()
    }
    fn parse_statement(&mut self) -> Statement {
        //TODO
        Statement::Expression(self.parse_expression())
    }
    pub fn produce_ast(&mut self) -> Program {
        let mut program = Program::new();

        while !self.eof() {
            program.body.push(self.parse_statement());
        }
        program
    }
}
