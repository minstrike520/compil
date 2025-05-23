#[derive(Debug, Clone, Copy)]
pub enum RuntimeValue {
    NullValue,
    NumberValue(i32),
    Bool(bool),
}
