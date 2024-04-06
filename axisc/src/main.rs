use std::num::ParseFloatError;
use std::str::FromStr;
use cli::Cli;
use clap::Parser;
use reader::read_file;
use reader::TokenScanner;
use lexer::TokenKind;
use crate::lexer::{Token, Keyword, Literal, SimpleToken};

mod cli;
mod reader;
mod lexer;
mod error;
mod colors;

fn main() {
    let args = Cli::parse();
    let string = read_file(&args.source);

    let mut scanner = TokenScanner::from_file_string(&string, &args.source);
    let mut tokens: Vec<Token> = vec![];
    let mut pos: (usize, usize) = (1, 1);

    loop {
        match scanner.peek() {
            None => break,
            Some('#') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Pound)))
            },
            Some('*') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Multiply)))
            },
            Some('-') => {
                scanner.advance(1);

                if scanner.peek().unwrap_or(&' ').eq(&'-') {
                    scanner.advance_line();
                    tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Comment)))
                } else if scanner.peek().unwrap_or(&' ').eq(&'>') {
                    scanner.advance(1);
                    tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Arrow)))
                } else {
                    tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Subtract)))
                }
            },
            Some('+') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Add)))
            },
            Some('<') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::LessThan)))
            },
            Some('>') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::GreaterThan)))
            },
            Some('=') => {
                scanner.advance(1);

                if scanner.peek().unwrap_or(&' ').eq(&'=') {
                    scanner.advance(1);
                    tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::DoubleEquals)))
                } else {
                    tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Equals)))
                }
            },
            Some(';') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Semicolon)))
            },
            Some('/') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Divide)))
            },
            Some('%') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Percent)))
            },
            Some('^') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Caret)))
            },
            Some(':') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Colon)))
            },
            Some('$') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::Dollar)))
            },
            Some('\n') => {
                scanner.advance(1);
                tokens.push(Token::new(pos, TokenKind::Simple(SimpleToken::LineFeed)))
            },
            Some('"') => {
                tokens.push(Token::new(pos, TokenKind::Literal(Literal::string_literal_from_scanner(&mut scanner))))
            },
            Some(c) => match c {
                ' ' | '\n' => {
                    scanner.advance(1);
                },
                _ => {
                    let word = scanner.advance_word();
                    let word_number: Result<f64, ParseFloatError> = f64::from_str(&word);

                    match word_number {
                        Ok(n) => tokens.push(Token::new(pos, TokenKind::Literal(Literal::Number(n)))),
                        Err(_) => match word.as_ref() {
                            "" => (),
                            "True" => tokens.push(Token::new(pos, TokenKind::Literal(Literal::Boolean(true)))),
                            "False" => tokens.push(Token::new(pos, TokenKind::Literal(Literal::Boolean(false)))),
                            "Into" => tokens.push(Token::new(pos, TokenKind::Keyword(Keyword::Into))),
                            "Let" => tokens.push(Token::new(pos, TokenKind::Keyword(Keyword::Let))),
                            "As" => tokens.push(Token::new(pos, TokenKind::Keyword(Keyword::As))),
                            "String" => tokens.push(Token::new(pos, TokenKind::Keyword(Keyword::String))),
                            "Number" => tokens.push(Token::new(pos, TokenKind::Keyword(Keyword::Number))),
                            "Array" => tokens.push(Token::new(pos, TokenKind::Keyword(Keyword::Array))),
                            "Boolean" => tokens.push(Token::new(pos, TokenKind::Keyword(Keyword::Boolean))),
                            _ => tokens.push(Token::new(pos, TokenKind::Identifier(word)))
                        }
                    }
                }
            }
        }

        pos = (scanner.line, scanner.column)
    }

    println!("{:?}", tokens);
}
