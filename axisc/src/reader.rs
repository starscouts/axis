use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::path::PathBuf;
use std::process::exit;
use std::str::Chars;

pub fn read_file(source: &PathBuf) -> String {
    let file = File::open(&source).unwrap_or_else(|e| {
        eprintln!("axisc: Unable to open {:?} for reading: {}", &source, e);
        exit(1)
    });
    let reader = BufReader::new(file);
    let mut string = String::from("");

    for line in reader.lines() {
        let mut line = line.unwrap();

        if let Some(char_index) = line.find("--") {
            line.truncate(char_index);
        }

        if !line.is_empty() {
            string.push_str(line.trim());
            string.push('\n');
        }
    }

    string
}

pub struct TokenScanner<'a> {
    chars: Peekable<Chars<'a>>
}

impl <'a> TokenScanner<'a> {
    pub fn advance(&mut self, num: usize) -> Option<char> {
        self.chars.nth(num)
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn advance_word(&mut self) -> String {
        let mut word = String::new();

        while let Some(char) = self.chars.next() {
            if char.is_ascii_whitespace() {
                break;
            }

            word.push(char);
        }

        word
    }

    pub fn peek_word(&mut self) -> String {
        todo!(":(")
    }

    pub fn from_string(string: &'a str) -> Self {
        Self {
            chars: string.chars().peekable()
        }
    }
}
