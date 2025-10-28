use crate::{frontend::parser::Parser, runtime::environment::Environment};

fn standard_test(content: &str) {
    let mut tokens = Parser::initialize(content.to_string());
    dbg!(&tokens);
    let program = tokens.produce_ast();
    dbg!(&program);
    let mut environment = Environment::create(None);
    let result = environment.evaluate_program(program).map(|x| ());
    dbg!(environment.clone());
    dbg!(&result);
}

fn test1() {
    let content = 
r#"a = 3; c = 5;
d = 1;
"#;
    standard_test(content);
}

#[test]
fn test2() {
    let content = 
r#"
let a; a = 3;
let c = 5; c = 4;
let d = a + c * 2;
"#;
    standard_test(content);
}
