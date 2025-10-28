use std::collections::VecDeque;

use super::{
    ast::{Expression, Program, Statement},
    lexer::{tokenize, BinaryOperator, Token},
};

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn initialize(source_code: String) -> Self {
        Self {
            tokens: VecDeque::from(tokenize(source_code)),
        }
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

        while matches!(
            self.at(),
            Token::BinaryOperator(BinaryOperator::Additive(_))
        ) {
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
        while matches!(
            self.at(),
            Token::BinaryOperator(BinaryOperator::Multiplicitave(_))
        ) {
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
                assert!(
                    self.pop_front() == Token::CloseParen,
                    "syntax error: Expected a close parenthesis"
                );
                expr
            }
            Token::CloseParen => {
                panic!("syntax error: A close parenthesis is missing a corresponding open one.")
            }
            _ => panic!("Not an expression token: {token}"),
        }
    }
    fn parse_variable_declaration(&mut self) -> Statement {
        assert!(self.pop_front() == Token::Let);
        let identifier = match self.pop_front() {
            Token::Identifier(i) => i,
            _ => panic!("syntax error: constant statement should be followed by an identifier."),
        };
        match self.pop_front() {
            Token::Semicolon => {
                Statement::VarDeclaration { identifier, value: None }
            },
            Token::Equals => {
                let value = Some(self.parse_expression());
                if self.pop_front() != Token::Semicolon {
                    panic!("syntax error: Constant declaration statement must end with semicolon.")
                }
                Statement::VarDeclaration { identifier, value }
            },
            t => panic!("syntax error: Not a valid constant assignment (expecting '=' or ';', but '{}' found)", t.to_string())
        }
    }
    fn parse_const_declaration(&mut self) -> Statement {
        assert!(self.pop_front() == Token::Const);
        let identifier = match self.pop_front() {
            Token::Identifier(i) => i,
            _ => panic!("syntax error: let statement should be followed by an identifier."),
        };
        match self.pop_front() {
            Token::Semicolon => {
                panic!("constant declaration should contain value")
            }
            Token::Equals => {
                let value = self.parse_expression();
                if self.pop_front() != Token::Semicolon {
                    panic!("syntax error: Variable declaration statement must end with semicolon.")
                }
                Statement::ConstDeclaration { identifier, value }
            }
            t => panic!(
                "syntax error: Not a valid let assignment (expecting '=' or ';', but '{}' found)",
                t.to_string()
            ),
        }
    }
    fn parse_variable_assignment(&mut self) -> Statement {
        let left = self.parse_expression();
        if *self.at() != Token::Equals {
            return Statement::Expression(left);
        }
        self.pop_front();
        let value = self.parse_expression();
        Statement::VarAssignment {
            assigne: left,
            value,
        }
    }
    fn parse_expression(&mut self) -> Expression {
        self.parse_additive_expression()
    }
    fn parse_statement(&mut self) -> Statement {
        match *self.at() {
            Token::Let => self.parse_variable_declaration(),
            Token::Const => self.parse_const_declaration(),
            Token::Identifier(_) => self.parse_variable_assignment(),
            _ => Statement::Expression(self.parse_expression()),
        }
    }
    pub fn produce_ast(&mut self) -> Program {
        let mut program = Program::new();

        while !self.eof() {
            program.body.push(self.parse_statement());
        }
        program
    }
}
