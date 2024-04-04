use cli::Cli;
use clap::Parser;
use reader::read_file;
use reader::TokenScanner;
use lexer::Token;

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
                tokens.push(Token::Pound)
            },
            Some('*') => {
                scanner.advance(1);
                tokens.push(Token::Multiply)
            },
            Some('-') => {
                scanner.advance(1);
                tokens.push(Token::Subtract)
            },
            Some('+') => {
                scanner.advance(1);
                tokens.push(Token::Add)
            },
            Some('<') => {
                scanner.advance(1);
                tokens.push(Token::LessThan)
            },
            Some('>') => {
                scanner.advance(1);
                tokens.push(Token::GreaterThan)
            },
            Some('=') => {
                scanner.advance(1);
                tokens.push(Token::Equals)
            },
            Some(';') => {
                scanner.advance(1);
                tokens.push(Token::Semicolon)
            },
            Some('/') => {
                scanner.advance(1);
                tokens.push(Token::Divide)
            },
            Some('%') => {
                scanner.advance(1);
                tokens.push(Token::Percent)
            },
            Some('^') => {
                scanner.advance(1);
                tokens.push(Token::Caret)
            },
            Some(':') => {
                scanner.advance(1);
                tokens.push(Token::Colon)
            },
            Some('$') => {
                scanner.advance(1);
                tokens.push(Token::Dollar)
            },
            Some('"') => {
                scanner.advance(1);
                tokens.push(Token::Quote)
            },
            Some('\n') => {
                scanner.advance(1);
                tokens.push(Token::LineFeed)
            },
            Some(_) => tokens.push(Token::Identifier(scanner.advance_word()))
        }
    }

    println!("{:?}", tokens);
}
