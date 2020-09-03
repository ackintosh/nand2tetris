use std::io::{BufReader, BufRead};
use std::fs::File;

// 表7-1 Parserモジュール
pub struct Parser {
    reader: BufReader<File>,
}

#[derive(Debug)]
pub enum Command {
    Arithmetic(Operator),
    Push(MemoryAccess),
    Pop(MemoryAccess),
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
    // アーキテクチャ上で物理領域を占有しない、仮想的な存在
    Constant,
    // RAMのベースアドレスを保持する
    Local,
    Argument,
    This,
    That,
    // RAM上の決められた領域に直接マッピングされている
    Pointer,
    Temp,
    // RAMアドレスの16番目から始まるスタティック変数
    Static,
}

impl MemorySegment {
    fn from(s: &str) -> Self {
        match s {
            "constant" => MemorySegment::Constant,
            "local" => MemorySegment::Local,
            "argument" => MemorySegment::Argument,
            "this" => MemorySegment::This,
            "that" => MemorySegment::That,
            "pointer" => MemorySegment::Pointer,
            "temp" => MemorySegment::Temp,
            "static" => MemorySegment::Static,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct MemoryAccess {
    pub segment: MemorySegment,
    pub index: u16,
}

impl MemoryAccess {
    fn from(segment: &str, index: u16) -> Self {
        let segment = MemorySegment::from(segment);
        match segment {
            MemorySegment::Pointer => {
                if index > 3 {
                    panic!(format!("index {} is not supported", index));
                }
            }
            MemorySegment::Temp => {
                if index > 7 {
                    panic!(format!("index {} is not supported", index));
                }
            }
            _ => {}
        }
        Self {
            segment,
            index,
        }
    }

    pub fn get_static_address(&self) -> u16 {
        match self.segment {
            MemorySegment::Pointer => 3 + self.index,
            MemorySegment::Temp => 5 + self.index,
            _ => unreachable!(),
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
                Command::Push(MemoryAccess::from(elems[1], elems[2].parse::<u16>().unwrap()))
            },
            "pop" => {
                assert_eq!(elems.len(), 3, "pop command requires 2 arguments");
                Command::Pop(MemoryAccess::from(elems[1], elems[2].parse::<u16>().unwrap()))
            }
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
