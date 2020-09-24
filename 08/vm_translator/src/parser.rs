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
    Label(String),
    IfGoto(String),
    Goto(String),
    Function(Function),
    Call(Call),
    Return,
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

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub num_local_variables: u16,
}

impl Function {
    fn new(name: String, num_local_variables: u16) -> Self {
        Self {
            name,
            num_local_variables,
        }
    }
}

#[derive(Debug)]
pub struct Call {
    pub function_name: String,
    pub num_arguments: u16,
}

impl Call {
    pub fn new(function_name: String, num_arguments: u16) -> Self {
        Self {
            function_name,
            num_arguments,
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

                // コメント以降を削除
                if let Some(pos) = buf.find("//") {
                    buf.replace_range(pos.., "");
                }

                // スペースを削除
                buf = String::from(buf.trim());

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
            "label" => {
                assert_eq!(elems.len(), 2, "label command requires 1 argument");
                Command::Label(String::from(elems[1]))
            }
            "if-goto" => {
                assert_eq!(elems.len(), 2, "if-goto command requires 1 argument");
                Command::IfGoto(String::from(elems[1]))
            }
            "goto" => {
                assert_eq!(elems.len(), 2, "goto command requires 1 argument");
                Command::Goto(String::from(elems[1]))
            }
            "function" => {
                assert_eq!(elems.len(), 3, "function command requires 2 arguments");
                Command::Function(Function::new(String::from(elems[1]), elems[2].parse::<u16>().unwrap()))
            }
            "call" => {
                assert_eq!(elems.len(), 3, "call command requires 2 arguments");
                Command::Call(Call::new(String::from(elems[1]), elems[2].parse::<u16>().unwrap()))
            }
            "return" => {
                assert_eq!(elems.len(), 1, "return command requires no arguments");
                Command::Return
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
