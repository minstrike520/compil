use std::{env, fs};

mod lexer;
mod ast;
mod parser;

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Cannot read user input");
    input
}

fn shell() {
    println!("Custom lang shell, v0.0.0");
    loop {
        print!("> ");
        let input = read_string();
        if input.eq("exit") {
            println!("Shell exits.");
            break;
        }
        let program = parser::Parser::initialize(input).produce_ast();
        println!("{:#?}", program);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].eq("shell") {
        shell();
        return
    }

    let file_path = &args[1];
    println!("In file '{}'", file_path);

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("<FILE CONTENT> \n{content}");
   
    let parse_result = parser::Parser::initialize(content).produce_ast();
    println!("parse result: {:?}", parse_result);
}
