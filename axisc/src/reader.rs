use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::path::PathBuf;
use std::process::exit;
use std::str::Chars;
use crate::lexer::Delimiter;

pub fn read_file(source: &PathBuf) -> String {
    let file = File::open(&source).unwrap_or_else(|e| {
        eprintln!("axisc: Unable to open {:?} for reading: {}", &source, e);
        exit(1)
    });
    let reader = BufReader::new(file);
    let mut string = String::from("");

    for line in reader.lines() {
        let line = line.unwrap();

        string.push_str(line.trim());
        string.push('\n');
    }

    string
}

pub struct TokenScanner<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub string: &'a str,
    pub line: usize,
    pub column: usize,
    pub file_name: &'a PathBuf
}

impl <'a> TokenScanner<'a> {
    pub fn advance(&mut self, num: usize) -> Option<char> {
        let ch = self.chars.nth(num - 1);

        if ch.map_or(false, |c| c == '\n') {
            self.line += 1;
            self.column = 1;
            ch
        } else {
            self.column += 1;
            ch
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn advance_word(&mut self) -> String {
        let mut word = String::new();

        while let Some(char) = self.chars.peek() {
            if Delimiter::is_delimiter(char) {
                break;
            }

            word.push(*char);
            self.column += 1;
            self.chars.next();
        }

        word
    }

    pub fn advance_line(&mut self) -> String {
        let mut word = String::new();

        while let Some(char) = self.chars.peek() {
            if char.eq(&'\n') {
                break;
            }

            word.push(*char);
            self.chars.next();
        }

        word
    }

    pub fn from_file_string(string: &'a str, file_name: &'a PathBuf) -> Self {
        Self {
            chars: string.chars().peekable(),
            string,
            column: 1,
            line: 1,
            file_name
        }
    }

    pub fn get_current_line(&self) -> &str {
        self.string.lines().nth(self.line - 1).unwrap_or_default()
    }

    pub fn get_line(&self, line: usize) -> &str {
        self.string.lines().nth(line - 1).unwrap_or_default()
    }
}
