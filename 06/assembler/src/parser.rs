use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::parser::Command::{ACommand, LCommand, CCommand};

// 6.3.1 Parserモジュール
pub struct Parser {
    reader: BufReader<File>,
    current_command: Option<Command>
}

enum Command {
    ACommand,
    CCommand,
    LCommand,
}

impl Parser {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
            current_command: None,
        }
    }

    // 表6-1にはadvanceルーチンがあるけど未実装
    // fn advance() {}

    pub fn has_more_commands(&mut self) -> bool {
        let mut buf = String::new();

        loop {
            if let Ok(len) = self.reader.read_line(&mut buf) {
                if len == 0 {
                    self.current_command = None;
                    return false;
                }
            } else {
                panic!();
            }

            let trimmed = buf.trim();
            if trimmed.len() == 0 {
                buf.clear();
                continue;
            }

            if trimmed.starts_with("//") {
                buf.clear();
                continue;
            }

            self.current_command = Some(Self::parse_command(trimmed));
            break;
        }

        return true;
    }

    fn parse_command(command: &str) -> Command {
        match command.chars().next().unwrap() {
            '@' => {
                ACommand
            }
            '(' => {
                LCommand
            }
            _ => {
                CCommand
            }
        }
    }
}
