use std::{env, fs, io::Write};

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
    let mut environment = Environment::create(None);
    environment
        .declare_variable("test_variable", RuntimeValue::NumberValue(3))
        .unwrap()
        .declare_variable("true", RuntimeValue::Bool(true))
        .unwrap()
        .declare_variable("false", RuntimeValue::Bool(false))
        .unwrap()
        .declare_variable("null", RuntimeValue::NullValue)
        .unwrap();
    loop {
        print!("> ");
        std::io::stdout().flush().expect("io flush err");
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
    if let Some(arg1) = args.get(1) {
        if arg1 != "shell" {
            return;
        }
        shell();
        return;
    }

    let file_path = &args[1];
    println!("In file '{}'", file_path);

    let content = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("<FILE CONTENT> \n{content}");

    let parse_result = frontend::parser::Parser::initialize(content).produce_ast();
    println!("parse result: {:?}", parse_result);
}
