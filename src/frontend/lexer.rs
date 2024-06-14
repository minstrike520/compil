use std::{collections::VecDeque, fmt::Display};

/// # Note: following rust document "Defining an Enum"
/// As the document says:
/// > However, representing the same concept using just an enum is more concise: rather than an
/// > enum inside a struct, we can put data directly into each enum variant.
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    Identifier(String),
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
    Additive(String),
    Multiplicitave(String)
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Additive(string) => string,
            Self::Multiplicitave(string) => string
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Number(string) => string.clone(),
            Self::Identifier(string) => string.clone(),
            Self::Equals => "=".to_string(),
            Self::OpenParen => "(".to_string(),
            Self::CloseParen => ")".to_string(),
            Self::BinaryOperator(binary_operator) => binary_operator.to_string(),
            Self::Let => "let".to_string(),
            Self::Null => "null".to_string(),
            Self::EOF => "<END OF FILE>".to_string(),
        })
    }
}

/// # Note: To use `String` as `&str`
/// It's actually quite easy...just use `String`'s `as_str`.
/// ```
/// let string = String::new("awa");
/// let static_str = string.as_str(); // type = &'static str
/// ```
pub fn find_reserved(token: &String) -> Option<Token> {
    match token.as_str() {
        "let" => Some(Token::Let),
        "null" => Some(Token::Null),
        _ => None,
    }
}

pub fn is_skippable(character: &char) -> bool {
    vec![
        ' ',
        '\n',
        '\t',
    ].contains(character)
}

fn match_token(characters: &mut VecDeque<char>) -> Option<Token> {
    Some(match characters[0] {
        '(' => { characters.pop_front(); Token::OpenParen }, 
        ')' => { characters.pop_front(); Token::CloseParen }, 
        '+' | '-' => Token::BinaryOperator(BinaryOperator::Additive(characters.pop_front().unwrap().to_string())),
        '*' | '/' | '%' => Token::BinaryOperator(BinaryOperator::Multiplicitave(characters.pop_front().unwrap().to_string())),

        '=' => { characters.pop_front(); Token::Equals },
        _ => {
            if characters[0].is_digit(10) {
                let mut number_token = String::new();
                while characters.len() > 0 && characters[0].is_digit(10) {
                    number_token += &characters.pop_front().unwrap().to_string();
                }
                Token::Number(number_token)
            }
            else if characters[0].is_alphabetic() {
                let mut identifier = String::new();
                while characters.len() > 0 && characters[0].is_alphabetic() {
                    identifier += &characters.pop_front().unwrap().to_string();
                }
                match find_reserved(&identifier) {
                    Some(t) => t,
                    None => Token::Identifier(identifier) 
                }
            }
            else if is_skippable(&characters[0]) {
                let _ = characters.pop_front();
                return None
            }
            else { panic!(
                "Undefined character: {c}", 
                c = characters.pop_front().unwrap().to_string());
            }
        }
    })
}

pub fn tokenize(source_code: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut src: VecDeque<char> = source_code.chars().collect();
    
    while !src.is_empty() {
        let token = match match_token(&mut src) { Some(t) => t, None => continue };
        tokens.push(token);
    }
    tokens.push(Token::EOF);
    tokens
}
