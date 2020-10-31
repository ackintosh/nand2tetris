use std::path::{Path, PathBuf};
use std::io::{Error, BufReader, BufRead};
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    assert_eq!(args.len(), 2, "Path to .jack file or a directory contains .jack file is required.");

    let path = Path::new(&args[1]);
    Analyzer::run(path);
}

struct Analyzer {
}

impl Analyzer {
    fn run(source: &Path) {
        let jack_files = Self::jack_files(source).expect(format!("failed to read the path: {:?}", source).as_str());

        if jack_files.len() == 0 {
            panic!(".jack file is required.");
        }

        println!("jack_files: {:?}", jack_files);

        for f in jack_files {
            let f = std::fs::File::open(f.clone()).expect(format!("failed to open .jack file: {:?}", f).as_str());
            let mut tokenizer = Tokenizer::new(f);
            let tokens = tokenizer.generate_tokens();
        }
    }

    fn jack_files(path: &Path) -> Result<Vec<PathBuf>, Error>{
        if path.is_dir() {
            Ok(
                std::fs::read_dir(path)?.filter(|f| {
                    if !f.is_ok() {
                        return false;
                    }
                    "jack" == f.as_ref().unwrap().path().extension().expect("should have an extension")
                }).map(|f| {
                    f.unwrap().path()
                }).collect()
            )
        } else {
            if path.extension().expect("should have an extension") == "jack" {
                Ok(vec![path.to_path_buf()])
            } else {
                Ok(vec![])
            }
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

struct Tokenizer {
    reader: JackReader,
}

impl Tokenizer {
    fn new(file: File) -> Self {
        Self {
            reader: JackReader { reader: BufReader::new(file) },
        }
    }

    fn generate_tokens(&mut self) -> Tokens {
        while let Some(line) = self.reader.read_line() {
            println!("line: {}", line);

            let words = line.split(" ").collect::<Vec<&str>>().iter()
                .map(|&word| {
                    word.trim()
                }).filter(|&word| {
                    word.len() > 0
                }).collect::<Vec<_>>();
            if words.len() == 0 {
                continue;
            }
            println!("words: {:?}", words);

            for w in words {
                let token = Self::parse(w);
                println!("token: {:?}", token);
            }
        }

        return Tokens { elements: vec![] }
    }

    fn parse(word: &str) -> Token {
        println!("w: {}", word);
        if KEYWORD.contains(&word) {
            return Token::Keyword(String::from(word));
        }

        if SYMBOL.contains(&word) {
            return Token::Symbol(String::from(word));
        }

        match word.chars().nth(0).expect("should have chars at least 1") {
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                return Token::IntConst(String::from(word));
            }
            '"' => {
                return Token::StringConst(String::from(word.trim_matches('"')));
            }
            _ => {
                todo!()
            }
        }

        Token::Identifier
    }
}

struct Tokens {
    elements: Vec<Token>,
}

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

#[derive(Debug)]
enum Token {
    Keyword(String),
    Symbol(String),
    Identifier,
    IntConst(String),
    StringConst(String),
}
