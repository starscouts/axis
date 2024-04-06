use std::fmt;
use std::process::exit;
use crate::colors::{_COLOR_FG_BLUE_BRIGHT, _COLOR_FG_RED, _FORMAT_BOLD, _FORMAT_RESET};
use crate::lexer::TokenPosition;
use crate::reader::TokenScanner;

pub fn raise_error(token_scanner: &TokenScanner, error: CompileError, base_hint: Option<&str>, hints: Option<ErrorHints>, note: Option<&str>) -> ! {
    let hints = hints.unwrap_or_default();
    let mut longest_line = token_scanner.line.to_string().len();

    for hint in &hints {
        let hint_line_length = hint.position.line.to_string().len();
        if hint_line_length > longest_line {
            longest_line = hint_line_length;
        }
    }

    println!("{_FORMAT_BOLD}{_COLOR_FG_RED}error{_FORMAT_RESET}{_FORMAT_BOLD}: [{:?}] {}", error, error);
    println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT}--> {}:{}:{}", " ".repeat(longest_line), token_scanner.file_name.display(), token_scanner.line, token_scanner.column);
    println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT} |", " ".repeat(longest_line));

    for hint in &hints {
        if hint.position.line < token_scanner.line {
            println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT}{} | {_FORMAT_RESET}{}", " ".repeat(longest_line - hint.position.line.to_string().len()), hint.position.line, token_scanner.get_line(hint.position.line));
            println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT} | {}- {}", " ".repeat(longest_line), " ".repeat(hint.position.column - 1), hint.message);
        }
    }

    println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT}{} | {_FORMAT_RESET}{}", " ".repeat(longest_line - token_scanner.line.to_string().len()), token_scanner.line, token_scanner.get_current_line());
    println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT} | {}{_FORMAT_BOLD}{_COLOR_FG_RED}^ {}", " ".repeat(longest_line), " ".repeat(token_scanner.column - 1), base_hint.unwrap_or(""));

    for hint in &hints {
        if hint.position.line > token_scanner.line {
            println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT}{} | {_FORMAT_RESET}{}", " ".repeat(longest_line - hint.position.line.to_string().len()), hint.position.line, token_scanner.get_line(hint.position.line));
            println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT} | {}- {}", " ".repeat(longest_line), " ".repeat(hint.position.column - 1), hint.message);
        }
    }

    let note = note.unwrap_or_default();
    if !note.is_empty() {
        println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT} |", " ".repeat(longest_line));
        println!("{}{_FORMAT_BOLD}{_COLOR_FG_BLUE_BRIGHT} = {_FORMAT_RESET}{_FORMAT_BOLD}note{_FORMAT_RESET}: {}", " ".repeat(longest_line), note);
    }

    println!();

    exit(-1)
}

#[derive(Debug)]
pub enum CompileError {
    UnexpectedStringEOF,
    UnexpectedStringLineBreak
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompileError::UnexpectedStringEOF => write!(f, "unexpected end of file while reading string"),
            CompileError::UnexpectedStringLineBreak => write!(f, "unexpected line break while reading string"),
        }
    }
}

pub type ErrorHints<'a> = Vec<ErrorHint<'a>>;

pub struct ErrorHint<'a> {
    pub position: TokenPosition,
    pub message: &'a str
}

impl <'a> ErrorHint<'a> {
    pub fn new(position: TokenPosition, message: &'a str) -> Self {
        Self {
            position,
            message
        }
    }
}
