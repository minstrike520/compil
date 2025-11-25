use std::collections::VecDeque;

use super::{
    ast::{Expression, Program, Statement},
    lexer::{tokenize, BinaryOperator, Token},
};

#[derive(Debug)]
pub struct Parser {
    tokens: VecDeque<Token>,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    SyntaxError(String),
    TypeError(String),
}

type ParseResult<T> = Result<T, ParseError>;

macro_rules! return_syntax_err {
    ($content:expr) => {
        return Err(ParseError::SyntaxError($content.to_string()))
    };
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
    fn parse_additive_expression(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_multiplicitave_expression()?;

        while matches!(
            self.at(),
            Token::BinaryOperator(BinaryOperator::Additive(_))
        ) {
            let operator = self.pop_front().to_string();
            let right = self.parse_multiplicitave_expression()?;
            left = Expression::BinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }
        }
        Ok(left)
    }
    fn parse_multiplicitave_expression(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_primary_expression()?;
        while matches!(
            self.at(),
            Token::BinaryOperator(BinaryOperator::Multiplicitave(_))
        ) {
            let operator = self.pop_front().to_string();
            let right = self.parse_primary_expression()?;
            left = Expression::BinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            }
        }
        Ok(left)
    }
    fn parse_primary_expression(&mut self) -> ParseResult<Expression> {
        let token = self.pop_front();
        Ok(match token {
            Token::Identifier(value) => Expression::Identifier(value),
            Token::Number(value) => Expression::NumericLiteral(value.parse::<i32>().unwrap()),
            Token::OpenParen => {
                let expr = self.parse_expression();
                if self.pop_front() != Token::CloseParen {
                    return_syntax_err!("expected a close parenthesis");
                }
                expr
            }
            Token::CloseParen => {
                return_syntax_err!("'(' was never closed");
            }
            _ => {
                return_syntax_err!(format!("'{token}' is not an expression token"));
            }
        })
    }
    fn parse_let(&mut self) -> ParseResult<Statement> {
        assert!(self.pop_front() == Token::Let);
        let identifier = match self.pop_front() {
            Token::Identifier(i) => i,
            _ => {
                return_syntax_err!("constant statement should be followed by an identifier")
            }
        };
        Ok(match self.pop_front() {
            Token::Semicolon => Statement::VarDeclaration {
                identifier,
                value: None,
            },
            Token::Equals => {
                let value = Some(self.parse_expression());
                if self.pop_front() != Token::Semicolon {
                    return_syntax_err!("constant declaration statement must end with semicolon");
                }
                Statement::VarDeclaration { identifier, value }
            }
            t => {
                return_syntax_err!(format!(
                    "not a valid constant assignment (expecting '=' or ';', but '{t}' found)"
                ));
            }
        })
    }
    fn parse_const(&mut self) -> ParseResult<Statement> {
        assert!(self.pop_front() == Token::Const);
        let identifier = match self.pop_front() {
            Token::Identifier(i) => i,
            _ => {
                return_syntax_err!("let statement should be followed by an identifier");
            } //panic!("syntax error: let statement should be followed by an identifier."),
        };
        match self.pop_front() {
            Token::Semicolon => {
                panic!("constant declaration should contain value")
            }
            Token::Equals => {
                let value = self.parse_expression();
                if self.pop_front() != Token::Semicolon {
                    return_syntax_err!("variable declaration statement must end with semicolon.");
                }
                Statement::ConstDeclaration { identifier, value }
            }
            t => return_syntax_err!(format!(
                "not a valid let assignment (expecting '=' or ';', but '{}' found)",
                t.to_string()
            )),
        }
    }
    fn parse_identifier(&mut self) -> Statement {
        let left = self.parse_expression();
        if *self.at() != Token::Equals {
            self.pop_front();
            return Statement::Expression(left);
        }
        self.pop_front();
        let value = self.parse_expression();
        if *self.at() == Token::Semicolon {
            self.pop_front();
        }
        assert!(matches!(left, Expression::Identifier(_)));
        if let Expression::Identifier(identifier) = left {
            Statement::VarAssignment { identifier, value }
        } else {
            panic!()
        }
    }
    fn parse_expression(&mut self) -> Expression {
        self.parse_additive_expression()
    }
    fn parse_statement(&mut self) -> Statement {
        match *self.at() {
            Token::Let => self.parse_let(),
            Token::Const => self.parse_const(),
            Token::Identifier(_) => self.parse_identifier(),
            _ => Statement::Expression(self.parse_expression()),
        }
    }
    pub fn produce_ast(&mut self) -> Program {
        let mut program = Program::new();

        while !self.eof() {
            let s = self.parse_statement();
            program.body.push(s);
        }
        program
    }
}
