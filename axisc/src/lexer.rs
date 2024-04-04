use crate::reader::TokenScanner;

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
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Array)
}

#[derive(Debug)]
pub enum Array {
    StringArray(Vec<String>),
    NumberArray(Vec<f64>),
    BooleanArray(Vec<bool>),
    MultiDimensionalArray(Vec<Array>)
}

impl Literal {
    pub fn string_literal_from_scanner(token_scanner: &mut TokenScanner) -> Literal {
        token_scanner.advance(1);
        let mut string = String::new();

        loop {
            match token_scanner.advance(1) {
                Some('"') => break,
                Some(c) => string.push(c),
                None => todo!("Reached EOF before end of string")
            }
        }

        Literal::String(string)
    }
}
