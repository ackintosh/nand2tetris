use std::io::{BufReader, BufRead};
use std::fs::File;

const KEYWORD: [&str; 21] = [
    "class",
    "constructor",
    "function",
    "method",
    "field",
    "static",
    "var",
    "int",
    "char",
    "boolean",
    "void",
    "true",
    "false",
    "null",
    "this",
    "let",
    "do",
    "if",
    "else",
    "while",
    "return",
];

const SYMBOL: [&str; 19] = [
    "{",
    "}",
    "(",
    ")",
    "[",
    "]",
    ".",
    ",",
    ";",
    "+",
    "-",
    "*",
    "/",
    "&",
    "|",
    "<",
    ">",
    "=",
    "~",
];

pub struct Tokenizer {
    reader: JackReader,
}

impl Tokenizer {
    pub fn new(file: File) -> Self {
        Self {
            reader: JackReader { reader: BufReader::new(file) },
        }
    }

    pub fn generate_tokens(&mut self) -> Vec<Token> {
        let mut elements = vec![];

        while let Some(line) = self.reader.read_line() {
            let words = Self::split_with_space(&line);
            let tokens = words.iter()
                .map(|word| {
                    word.trim()
                })
                .filter(|&word| {
                    word.len() > 0
                })
                .map(|word| {
                    Self::split_with_symbol(word)
                })
                .flatten()
                .map(|word| {
                    Self::parse(&word)
                })
                .collect::<Vec<_>>();
            elements.push(tokens);
        }

        return elements.into_iter().flatten().collect::<Vec<Token>>();
    }

    fn split_with_space(line: &str) -> Vec<String> {
        let mut words = vec![];
        let mut buf = vec![];
        let mut is_string = false;

        for c in line.chars() {
            match c {
                ' ' => {
                    if is_string {
                        buf.push(c.to_string());
                    } else {
                        words.push(buf.join(""));
                        buf.clear();
                    }
                },
                '"' => {
                    if is_string {
                        buf.push(c.to_string());
                        words.push(buf.join(""));
                        buf.clear();
                        is_string = false;
                    } else {
                        is_string = true;
                        buf.push(c.to_string());
                    }
                },
                other => {
                    buf.push(other.to_string());
                }
            }
        }

        if buf.len() > 0 {
            words.push(buf.join(""));
        }

        words
    }

    fn split_with_symbol(word: &str) -> Vec<String> {
        let mut i = 0;
        let mut parts: Vec<String> = vec![];
        let mut buf: Vec<char> = vec![];
        while i < word.len() {
            let c = word.chars().nth(i).unwrap();
            if SYMBOL.contains(&c.to_string().as_str()) {
                if buf.len() > 0 {
                    parts.push(buf.iter().map(|c| c.to_string() ).collect::<Vec<_>>().join(""));
                    buf.clear();
                }
                parts.push(String::from(c.to_string()))
            } else {
                buf.push(c);
            }
            i += 1;
        }
        if buf.len() > 0 {
            parts.push(buf.iter().map(|c| c.to_string() ).collect::<Vec<_>>().join(""));
        }

        parts
    }

    fn parse(word: &str) -> Token {
        if KEYWORD.contains(&word) {
            return Token::Keyword(String::from(word));
        }

        if SYMBOL.contains(&word) {
            return Token::Symbol(String::from(word));
        }

        match word.chars().nth(0).expect("should have chars at least 1") {
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                return Token::IntegerConst(String::from(word));
            }
            '"' => {
                return Token::StringConst(String::from(word.trim_matches('"')));
            }
            _ => {
                return Token::Identifier(String::from(word));
            }
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Symbol(String),
    Identifier(String),
    IntegerConst(String),
    StringConst(String),
}

impl From<&Token> for Token {
    fn from(token: &Token) -> Self {
        match token {
            Token::Keyword(s) => Token::Keyword(String::from(s)),
            Token::Symbol(s) => Token::Symbol(String::from(s)),
            Token::Identifier(s) => Token::Identifier(String::from(s)),
            Token::IntegerConst(s) => Token::IntegerConst(String::from(s)),
            Token::StringConst(s) => Token::StringConst(String::from(s)),
        }
    }
}

struct JackReader {
    reader: BufReader<File>
}

impl JackReader {
    fn read_line(&mut self) -> Option<String> {
        let mut buf = String::new();

        loop {
            if let Ok(len) = self.reader.read_line(&mut buf) {
                if len == 0 {
                    // EOF
                    return None;
                }

                // コメント以降を削除
                if let Some(pos) = buf.find("//") {
                    buf.replace_range(pos.., "");
                }

                if let Some(pos_start) = buf.find("/*") {
                    let pos_end = {
                        let s = &buf[pos_start..];
                        if let Some(pos_end) = s.find("*/") {
                            Some(pos_start + pos_end + 2) // "*/" の分を +2 している
                        } else {
                            None
                        }
                    };
                    if pos_end.is_some() {
                        buf.replace_range(pos_start..pos_end.unwrap(    ), "");
                    }
                }

                let buf = String::from(buf.trim());

                if buf.len() == 0 {
                    continue;
                }

                return Some(buf);
            } else {
                panic!("failed to read line");
            }
        }
    }
}
