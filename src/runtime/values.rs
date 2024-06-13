#[derive(Debug)]
pub enum RuntimeValue {
    NullValue(NullValue),
    NumberValue(NumberValue)
}

#[derive(Debug)]
pub struct NullValue;

#[derive(Debug)]
pub struct NumberValue {
    pub value: i32
}

impl NumberValue {
    pub fn create(value: i32) -> Self { Self{ value } }
}
