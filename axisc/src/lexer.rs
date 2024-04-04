#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Literal(Literal),
    Multiply,
    Divide,
    Pound,
    Colon,
    Percent,
    Caret,
    Dollar,
    Equals,
    Semicolon,
    GreaterThan,
    LessThan,
    Add,
    Subtract,
    LineFeed,
    Quote,
}

pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Array)
}

pub enum Array {
    StringArray(Vec<String>),
    NumberArray(Vec<f64>),
    BooleanArray(Vec<bool>),
    MultiDimensionalArray(Vec<Array>)
}
