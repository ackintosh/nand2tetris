use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};
use crate::parser::Command::{ACommand, LCommand, CCommand};
use crate::symbol_table::SymbolTable;

// 6.3.1 Parserモジュール
// 主な機能は各アセンブリコマンドをその基本要素に分解すること
pub struct Parser {
    reader: BufReader<File>,
    symbol_table: SymbolTable,
    next_rom_address: u16,
}

#[derive(Debug)]
pub enum Command {
    ACommand(String),
    CCommand((String, String, String)), // dest, comp, jump
    LCommand(String),
}

impl Parser {
    pub fn new(file: File) -> Self {
        Self {
            reader: BufReader::new(file),
            symbol_table: SymbolTable::new(),
            next_rom_address: 0,
        }
    }

    // 表6-1にはあるけど未実装
    // has_more_commands
    // command_type
    // symbol
    // dest
    // comp
    // jump

    fn read_line(&mut self) -> Option<String> {
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

            // スペースを削除
            buf.retain(|c| !c.is_whitespace());

            // コメント以降を削除
            if let Some(pos) = buf.find("//") {
                buf.replace_range(pos.., "");
            }

            if buf.len() == 0 {
                buf.clear();
                continue;
            }

            return Some(buf);
        }
    }

    pub fn advance(&mut self) -> Option<Command> {
        loop {
            let command_string = self.read_line()?;

            match self.parse_command(command_string.as_str()) {
                LCommand(_label) => {
                    // nop
                    continue
                },
                command => {
                    // A命令, C命令が読み込まれるROMアドレスを加算していく
                    // self.next_rom_address += 1;

                    return Some(command)
                }
            }
        }
    }

    // ラベルをシンボルテーブルに登録するために一度アセンブリプログラム全体をパースする
    pub fn scan_labels(&mut self) {
        loop {
            if let Some(command_string) = self.read_line() {
                match self.parse_command(command_string.as_str()) {
                    LCommand(label) => {
                        // シンボルテーブルに登録する
                        self.symbol_table.add(label.as_str(), self.next_rom_address);
                        continue;
                    },
                    _ => {
                        // A命令, C命令が読み込まれるROMアドレスを加算していく
                        self.next_rom_address += 1;
                        continue;
                    }
                }
            } else {
                break;
            }
        }

        self.reader.seek(SeekFrom::Start(0)).expect("failed to seek");
    }

    fn parse_command(&mut self, command: &str) -> Command {
        match command.chars().next().unwrap() {
            '@' => {
                let mut command_string = String::new();
                let mut chars = command.chars();
                chars.next(); // @を除外する
                while let Some(c) = chars.next() {
                    command_string.push(c);
                }

                let command = command_string.parse::<u16>().unwrap_or_else(|_c| {
                    if let Some(address) = self.symbol_table.address(command_string.as_str()) {
                        *address
                    } else {
                        self.symbol_table.new_symbol(command_string.as_str())
                    }
                });

                ACommand(format!("{}", command))
            }
            '(' => {
                let mut label_string = String::new();
                let mut chars = command.chars();
                chars.next(); // '(' を除外する
                while let Some(c) = chars.next() {
                    if c == ')' {
                        break;
                    }
                    label_string.push(c);
                }
                LCommand(label_string)
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
