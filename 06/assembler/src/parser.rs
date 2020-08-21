use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::parser::Command::{ACommand, LCommand, CCommand};

// 6.3.1 Parserモジュール
pub struct Parser {
    reader: BufReader<File>,
}

#[derive(Debug)]
pub enum Command {
    ACommand(String),
    CCommand(String),
    LCommand,
}

impl Parser {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }

    // 表6-1にはあるけど未実装
    // has_more_commmands
    // command_type
    // symbol
    // dest
    // comp
    // jump

    pub fn advance(&mut self) -> Option<Command> {
        let mut buf = String::new();

        loop {
            if let Ok(len) = self.reader.read_line(&mut buf) {
                // EOF
                if len == 0 {
                    return None;
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

            return Some(Self::parse_command(trimmed));
        }
    }

    fn parse_command(command: &str) -> Command {
        match command.chars().next().unwrap() {
            '@' => {
                let mut command_string = String::new();
                let mut chars = command.chars();
                chars.next(); // @を除外する
                while let Some(c) = chars.next() {
                    command_string.push(c);
                }

                ACommand(command_string)
            }
            '(' => {
                // TODO
                LCommand
            }
            _ => {
                let mut command_string = String::new();
                let mut chars = command.chars();
                while let Some(c) = chars.next() {
                    command_string.push(c);
                }

                CCommand(command_string)
            }
        }
    }
}
