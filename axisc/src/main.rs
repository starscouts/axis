use std::num::ParseFloatError;
use std::str::FromStr;
use cli::Cli;
use clap::Parser;
use reader::read_file;
use reader::TokenScanner;
use lexer::Token;
use crate::lexer::{Keyword, Literal, SimpleToken};

mod cli;
mod reader;
mod lexer;

fn main() {
    let args = Cli::parse();
    let string = read_file(&args.source);

    let mut scanner = TokenScanner::from_string(&string);
    let mut tokens: Vec<Token> = vec![];

    loop {
        match scanner.peek() {
            None => break,
            Some('#') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Pound))
            },
            Some('*') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Multiply))
            },
            Some('-') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Subtract))
            },
            Some('+') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Add))
            },
            Some('<') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::LessThan))
            },
            Some('>') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::GreaterThan))
            },
            Some('=') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Equals))
            },
            Some(';') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Semicolon))
            },
            Some('/') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Divide))
            },
            Some('%') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Percent))
            },
            Some('^') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Caret))
            },
            Some(':') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Colon))
            },
            Some('$') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::Dollar))
            },
            Some('\n') => {
                scanner.advance(1);
                tokens.push(Token::SimpleToken(SimpleToken::LineFeed))
            },
            Some('"') => {
                tokens.push(Token::Literal(Literal::string_literal_from_scanner(&mut scanner)))
            },
            Some(' ' | '\n') => {
                scanner.advance(1);
            },
            Some(_) => {
                let word = scanner.advance_word();
                let word_number: Result<f64, ParseFloatError> = f64::from_str(&word);

                match word_number {
                    Ok(n) => tokens.push(Token::Literal(Literal::Number(n))),
                    Err(_) => match word.as_ref() {
                        "" => (),
                        "True" => tokens.push(Token::Literal(Literal::Boolean(true))),
                        "False" => tokens.push(Token::Literal(Literal::Boolean(false))),
                        "Into" => tokens.push(Token::Keyword(Keyword::Into)),
                        "Let" => tokens.push(Token::Keyword(Keyword::Let)),
                        "As" => tokens.push(Token::Keyword(Keyword::As)),
                        "String" => tokens.push(Token::Keyword(Keyword::String)),
                        "Number" => tokens.push(Token::Keyword(Keyword::Number)),
                        "Array" => tokens.push(Token::Keyword(Keyword::Array)),
                        "Boolean" => tokens.push(Token::Keyword(Keyword::Boolean)),
                        _ => tokens.push(Token::Identifier(word))
                    }
                }
            }
        }
    }

    println!("{:?}", tokens);
}
