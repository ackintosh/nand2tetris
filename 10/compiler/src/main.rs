use std::path::{Path, PathBuf};
use std::io::{Error, BufReader, BufRead, BufWriter, Write};
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

        for jack_file in jack_files {
            let f = std::fs::File::open(jack_file.clone()).expect(format!("failed to open .jack file: {:?}", jack_file).as_str());
            let mut tokenizer = Tokenizer::new(f);
            let tokens = tokenizer.generate_tokens();
            let destination = Self::source_to_destination(&jack_file);
            println!("destination: {:?}", destination);
            save(destination, tokens);
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

    fn source_to_destination(path: &PathBuf) -> PathBuf {
        let mut  dest = path.clone();
        dest.set_extension("xml");
        dest
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
        let mut elements = vec![];

        while let Some(line) = self.reader.read_line() {
            println!("line: {}", line);

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
            println!("tokens: {:?}", tokens);
            elements.push(tokens);
        }

        return Tokens { elements: elements.into_iter().flatten().collect::<Vec<Token>>() }
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
        println!("w: {}", word);
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
    Identifier(String),
    IntegerConst(String),
    StringConst(String),
}

fn save(destination: PathBuf, tokens: Tokens) {
    let mut writer = BufWriter::new(File::create(destination).expect("failed to create a file"));
    writer.write_all(b"<tokens>\n");
    for token in tokens.elements {
        match token {
            Token::Keyword(keyword) => writer.write_all(format!("<keyword>{}</keyword>\n", keyword).as_bytes()),
            Token::Symbol(symbol) => writer.write_all(format!("<symbol>{}</symbol>\n", convert_to_xml_symbol(&symbol)).as_bytes()),
            Token::Identifier(identifier) => writer.write_all(format!("<identifier>{}</identifier>\n", identifier).as_bytes()),
            Token::IntegerConst(integer_const) => writer.write_all(format!("<integerConstant>{}</integerConstant>\n", integer_const).as_bytes()),
            Token::StringConst(string_const) => writer.write_all(format!("<stringConstant>{}</stringConstant>\n", string_const).as_bytes()),
        };
    }
    writer.write_all(b"</tokens>");
}

fn convert_to_xml_symbol(symbol: &str) -> String {
    match symbol {
        "<" => "&lt;".to_owned(),
        ">" => "&gt;".to_owned(),
        "&" => "&amp;".to_owned(),
        other => String::from(other),
    }
}
