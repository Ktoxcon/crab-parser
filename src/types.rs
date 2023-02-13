#[derive(Debug)]
pub enum NumericValue<T> {
    NumberValue(T),
}

#[derive(Debug)]
pub enum LiteralValues {
    Null,
    Boolean(bool),
}

pub enum ComplexValues {
    Array,
    Object,
    NumericValue,
    StringValue(String),
}

#[derive(Debug, Clone)]
pub enum JsonValue {
    LiteralValues,
    ComplexValues,
}
