use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::parser::Command::{ACommand, LCommand, CCommand};

// 6.3.1 Parserモジュール
// 主な機能は各アセンブリコマンドをその基本要素に分解すること
pub struct Parser {
    reader: BufReader<File>,
}

#[derive(Debug)]
pub enum Command {
    ACommand(String),
    CCommand((String, String, String)), // dest, comp, jump
    LCommand,
}

impl Parser {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
        }
    }

    // 表6-1にはあるけど未実装
    // has_more_commands
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

                CCommand(Self::parse_c_command(command_string))
            }
        }
    }

    fn parse_c_command(s: String) -> (String, String, String) {
        let fragments: Vec<&str> = s.split('=').collect();
        let (dest, s): (String, &str) = match fragments.len() {
            2 => {
                (
                    String::from(*fragments.first().expect("")),
                    fragments.get(1).expect("")
                )
            }
            1 => {
                (
                    "null".to_owned(),
                    fragments.first().unwrap()
                )
            }
            _ => panic!()
        };

        let fragments: Vec<&str> = s.split(';').collect();
        let (comp, jump): (String, String) = match fragments.len() {
            2 => {
                (
                    String::from(*fragments.first().expect("")),
                    String::from(*fragments.get(1).expect(""))
                )
            }
            1 => {
                (
                    String::from(*fragments.first().unwrap()),
                    "null".to_owned()
                )
            }
            _ => panic!()
        };

        assert_ne!(dest.len() + comp.len() + jump.len() , 0);

        (dest, comp, jump)
    }
}
