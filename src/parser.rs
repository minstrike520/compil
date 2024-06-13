use std::collections::VecDeque;

use crate::{ast::{BinaryExpression, Expression, Identifier, NumericLiteral, Program, Statement}, lexer::{tokenize, BinaryOperator, Token, TokenType}};

pub struct Parser {
    tokens: VecDeque<Token>
}

impl Parser {
    pub fn initialize(source_code: String) -> Self {
        Self{ tokens: VecDeque::from(tokenize(source_code)) }
    }
    fn eof(&self) -> bool {
        self.tokens[0].r#type == TokenType::EOF
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
        let mut left = self.parse_primary_expression();

        while self.at().r#type == TokenType::BinaryOperator(BinaryOperator::Additive) {
            let operator = self.pop_front().value;
            let right = self.parse_primary_expression();
            left = Expression::BinaryExpression(BinaryExpression {
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
                operator,
            })
        }
        left
    }
    fn parse_primary_expression(&mut self) -> Expression {
        let token = self.pop_front();
        match token.r#type {
            TokenType::Identifier => Expression::Identifier(Identifier::create(token.value)),
            TokenType::Number => Expression::NumericLiteral(NumericLiteral::create(token.value.parse::<i32>().unwrap())),
            _ => {
                panic!("Unexpected token found during parsing: {}", token.value)
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
