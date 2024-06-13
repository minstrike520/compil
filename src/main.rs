mod lexer;

fn test() {
    lexer::tokenize("print(asdf)".to_string());
}

fn main() {
    test();
    println!("Hello, world!");
}
