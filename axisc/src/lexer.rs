use std::fmt;
use crate::error::{CompileError, ErrorHint, ErrorHints, raise_error};
use crate::reader::TokenScanner;

#[derive(Debug)]
pub enum TokenKind {
    Identifier(String),
    Keyword(Keyword),
    Literal(Literal),
    Simple(SimpleToken)
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Identifier(s) => write!(f, "`{}`", s),
            TokenKind::Keyword(k) => write!(f, "{:?}", k),
            TokenKind::Literal(l) => write!(f, "{}", l),
            TokenKind::Simple(t) => write!(f, "{}", t)
        }
    }
}

#[derive(Debug)]
pub enum Delimiter {
    WhiteSpace,
    Semicolon,
    LineFeed,
    Equals,
    Invalid
}

impl Delimiter {
    pub fn is_delimiter(c: &char) -> bool {
        matches!(c, ' ' | ';' | '\n' | '=')
    }

    pub fn from(c: char) -> Self {
        match c {
            ' ' => Delimiter::WhiteSpace,
            ';' => Delimiter::Semicolon,
            '\n' => Delimiter::LineFeed,
            '=' => Delimiter::Equals,
            _ => Delimiter::Invalid
        }
    }
}

#[derive(Debug)]
pub enum SimpleToken {
    Multiply,
    Divide,
    Pound,
    Colon,
    Percent,
    Caret,
    Dollar,
    Equals,
    DoubleEquals,
    Semicolon,
    GreaterThan,
    LessThan,
    Add,
    Subtract,
    LineFeed,
    Comment,
    Arrow
}

impl fmt::Display for SimpleToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SimpleToken::Multiply => write!(f, "multiply"),
            SimpleToken::Divide => write!(f, "divide"),
            SimpleToken::Pound => write!(f, "pound"),
            SimpleToken::Colon => write!(f, "colon"),
            SimpleToken::Percent => write!(f, "percent"),
            SimpleToken::Caret => write!(f, "caret"),
            SimpleToken::Dollar => write!(f, "dollar"),
            SimpleToken::Equals => write!(f, "equals"),
            SimpleToken::DoubleEquals => write!(f, "double equals"),
            SimpleToken::Semicolon => write!(f, "semicolon"),
            SimpleToken::GreaterThan => write!(f, "greater than"),
            SimpleToken::LessThan => write!(f, "less than"),
            SimpleToken::Add => write!(f, "plus"),
            SimpleToken::Subtract => write!(f, "minus "),
            SimpleToken::LineFeed => write!(f, "line break"),
            SimpleToken::Comment => write!(f, "comment"),
            SimpleToken::Arrow => write!(f, "arrow"),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Array)
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::String(_) => write!(f, "string"),
            Literal::Number(_) => write!(f, "number"),
            Literal::Boolean(_) => write!(f, "boolean"),
            Literal::Array(a) => write!(f, "{}", a),
        }
    }
}

#[derive(Debug)]
pub enum Keyword {
    Into,
    Boolean,
    Array,
    Number,
    String,
    As,
    Let,
    Free,
    Derive,
    Checkpoint,
    Subroutine,
    GoTo,
    AsFn,
    If,
    Then,
    Else,
    And,
    Or,
    Xor,
    Not,
    For,
    To,
    Next,
    While,
    Forever,
    Repeat,
}

#[derive(Debug)]
pub enum Array {
    String(Vec<String>),
    Number(Vec<f64>),
    Boolean(Vec<bool>),
    MultiDimensional(Vec<Array>)
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Array::String(_) => write!(f, "string array"),
            Array::Number(_) => write!(f, "number array"),
            Array::Boolean(_) => write!(f, "boolean array"),
            Array::MultiDimensional(_) => write!(f, "multi-dimensional array"),
        }
    }
}

impl Literal {
    pub fn string_literal_from_scanner(token_scanner: &mut TokenScanner) -> Literal {
        token_scanner.advance(1);

        let start_position = TokenPosition::from_scanner(token_scanner);
        let start_position_hint = ErrorHint::new(start_position, "string starts here");
        let start_position_hints = ErrorHints::from(vec![start_position_hint]);

        let mut string = String::new();

        loop {
            match token_scanner.advance(1) {
                Some('"') => break,
                Some('\n') => raise_error(token_scanner, CompileError::UnexpectedStringLineBreak, Some("line break occurs here"), Some(start_position_hints), Some("strings are wrapped in \"double quotes\"")),
                Some(c) => string.push(c),
                None => raise_error(token_scanner, CompileError::UnexpectedStringEOF, Some("file ends here"), Some(start_position_hints), Some("strings are wrapped in \"double quotes\"")),
            }
        }

        Literal::String(string)
    }
}

#[derive(Debug)]
pub struct Token {
    position: TokenPosition,
    token: TokenKind
}

impl Token {
    pub fn new(position: (usize, usize), token: TokenKind) -> Self {
        Self {
            position: TokenPosition {
                line: position.0,
                column: position.1
            },
            token
        }
    }
}

#[derive(Debug)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize
}

impl TokenPosition {
    pub fn from_scanner(token_scanner: &TokenScanner) -> Self {
        Self {
            line: token_scanner.line,
            column: token_scanner.column
        }
    }
}
