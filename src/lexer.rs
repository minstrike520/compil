use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number,
    Identifier,
    Equals,
    OpenParen,
    CloseParen,
    BinaryOperator(BinaryOperator),
    Let,
    Null,
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Additive, Multiplicitave
}

pub fn find_reserved(token: &String) -> Option<TokenType> {
    match token.as_str() {
        "let" => Some(TokenType::Let),
        "null" => Some(TokenType::Null),
        _ => None,
    }
}

pub fn is_skippable(token: &char) -> bool {
    vec![
        ' ',
        '\n',
        '\t',
    ].contains(token)
}

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub r#type: TokenType,
}

impl Token {
    pub fn new(value: String, r#type: TokenType) -> Self {
        Self { value, r#type }
    }
}

pub fn tokenize(source_code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut src: VecDeque<char> = source_code.chars().collect();
    
    while !src.is_empty() {
        let token: (String, TokenType) = match src[0] {
            '(' => (src.pop_front().unwrap().to_string(), TokenType::OpenParen),
            ')' => (src.pop_front().unwrap().to_string(), TokenType::CloseParen),
            '+' | '-' => (src.pop_front().unwrap().to_string(), TokenType::BinaryOperator(BinaryOperator::Additive)),
            '*' | '/' | '%' => (src.pop_front().unwrap().to_string(), TokenType::BinaryOperator(BinaryOperator::Multiplicitave)),

            '=' => (src.pop_front().unwrap().to_string(), TokenType::Equals),
            _ => {
                if src[0].is_digit(10) {
                    let mut number = String::new();
                    while src.len() > 0 && src[0].is_digit(10) {
                        number += &String::from(src.pop_front().unwrap());
                    }
                    (number, TokenType::Number)
                }
                else if src[0].is_alphabetic() {
                    let mut identifier = String::new();
                    while src.len() > 0 && src[0].is_alphabetic() {
                        identifier += &String::from(src.pop_front().unwrap());
                    }
                    match find_reserved(&identifier) {
                        Some(t) => (identifier, t),
                        None => (identifier, TokenType::Identifier) 
                    }
                }
                else if is_skippable(&src[0]) {
                    let _ = src.pop_front();
                    continue;
                }
                else { panic!(
                    "Undefined character: {c}", 
                    c = src.pop_front().unwrap().to_string());
                }
            }
        };
        tokens.push(Token::new(token.0, token.1));
    }
    tokens.push(Token::new("<END OF FILE>".to_string(), TokenType::EOF));
    tokens
}
