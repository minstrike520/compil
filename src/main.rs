use std::{env, fs};

mod lexer;
mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("In file '{}'", file_path);

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("<FILE CONTENT> \n{content}");
   
    let parse_result = lexer::tokenize(content);

    println!("parse result: {:?}", parse_result);
}
