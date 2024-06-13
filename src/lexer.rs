pub enum TokenType {
    Number,
    Identifier,
    Equals,
    OpenParen,
    CloseParen,
    BinaryOperator,
    Let,
}

pub fn find_reserved(token: &String) -> Option<TokenType> {
    match token.as_str() {
        "let" => Some(TokenType::Let),
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
    let mut src: Vec<char> = source_code.chars().collect();
    
    while !src.is_empty() {
        use TokenType::*;
        let token: (String, TokenType) = match src[0] {
            '(' => (src.pop().unwrap().to_string(), OpenParen),
            ')' => (src.pop().unwrap().to_string(), CloseParen),
            '+' | '-' | '*' | '/' => (src.pop().unwrap().to_string(), BinaryOperator),
            '=' => (src.pop().unwrap().to_string(), Equals),
            _ => {
                if src[0].is_digit(10) {
                    let mut number = String::new();
                    while src.len() > 0 && src[0].is_digit(10) {
                        number += &String::from(src.pop().unwrap());
                    }
                    (number, Number)
                }
                else if src[0].is_alphabetic() {
                    let mut identifier = String::new();
                    while src.len() > 0 && src[0].is_digit(10) {
                        identifier += &String::from(src.pop().unwrap());
                    }
                    match find_reserved(&identifier) {
                        Some(t) => (identifier, t),
                        None => (identifier, Identifier) 
                    }
                }
                else if is_skippable(&src[0]) {
                    let _ = src.pop();
                    continue;
                }
                else { panic!(
                    "Undefined character: {c}", 
                    c = src.pop().unwrap().to_string());
                }
            }
        };
        tokens.push(Token::new(token.0, token.1));
    }
    tokens
}
