use std::io::{BufReader, BufRead};
use std::fs::File;

// 表7-1 Parserモジュール
pub struct Parser {
    reader: BufReader<File>,
}

#[derive(Debug)]
pub enum Command {
    Arithmetic(Operator),
    Push(MemorySegment, u16), // segment, index
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl Operator {
    fn from(s: &str) -> Option<Self> {
        match s {
            "add" => Some(Operator::Add),
            "sub" => Some(Operator::Sub),
            "neg" => Some(Operator::Neg),
            "eq" => Some(Operator::Eq),
            "gt" => Some(Operator::Gt),
            "lt" => Some(Operator::Lt),
            "and" => Some(Operator::And),
            "or" => Some(Operator::Or),
            "not" => Some(Operator::Not),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum MemorySegment {
    Constant,
}

impl MemorySegment {
    fn from(s: &str) -> Self {
        match s {
            "constant" => MemorySegment::Constant,
            _ => panic!(),
        }
    }
}

impl Parser {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }

    pub fn advance(&mut self) -> Option<Command> {
        let mut buf = String::new();

        loop {
            if let Ok(len) = self.reader.read_line(&mut buf) {
                if len == 0 {
                    // EOF
                    return None;
                }

                // スペースを削除
                buf = String::from(buf.trim());

                // コメント以降を削除
                if let Some(pos) = buf.find("//") {
                    buf.replace_range(pos.., "");
                }

                if buf.len() == 0 {
                    continue;
                }

                return Some(Self::parse(buf.as_str()));
            } else {
                panic!();
            }
        }
    }

    fn parse(s: &str) -> Command {
        let elems: Vec<&str> = s.split(" ").collect();

        match elems[0] {
            "push" => {
                assert_eq!(elems.len(), 3, "push command requires 2 arguments");
                Command::Push(MemorySegment::from(elems[1]), elems[2].parse::<u16>().unwrap())
            },
            other => {
                if let Some(arithmetic_operator) = Operator::from(other) {
                    assert_eq!(elems.len(), 1, "arithmetic command requires no arguments");
                    return Command::Arithmetic(arithmetic_operator);
                }

                panic!(format!("the operator is not supported: {}", other));
            }
        }
    }
}
