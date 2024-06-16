use std::{env, fs};

use crate::runtime::{environment::Environment, values::RuntimeValue};

mod frontend;
mod runtime;

fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Cannot read user input");
    input
}

fn shell() {
    println!("Custom lang shell, v0.0.0");
    let mut environment = Environment::create(None)
        .declare_variable(&"test_variable".to_string(), RuntimeValue::NumberValue(3)).unwrap();
    loop {
        print!("> ");
        let input = read_string();
        if input == "exit\n".to_string() {
            println!("Shell exits.");
            break;
        }
        let program = frontend::parser::Parser::initialize(input).produce_ast();
        let result = runtime::interpreter::evaluate_program(program, &mut environment);
        println!("{:#?}", result);
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
   
    let parse_result = frontend::parser::Parser::initialize(content).produce_ast();
    println!("parse result: {:?}", parse_result);
}
