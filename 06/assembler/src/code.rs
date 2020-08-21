// 6.3.2 Codeモジュール
// Hackアセンブリ言語のニーモニックをバイナリコードへ変換する

use crate::parser::Command;

pub fn code(command: Command) -> [bool; 16] {
    match command {
        // A命令
        // 0vvvv vvvv vvvv vvvv
        Command::ACommand(s) => {
            // A命令のvalue部分を、2進数の文字列に変換
            let bits_string = format!("{:b}", s.parse::<u16>().unwrap());

            let mut bitarray = [false; 16];
            let mut count = 15;
            for b in bits_string.chars().rev() {
                if b == '1' {
                    bitarray[count] = true
                } else {
                    bitarray[count] = false
                }
                count -= 1;
            }

            bitarray
        }
        // C命令
        // 111a cccc ccdd djjj
        Command::CCommand(mnemonics) => {
            println!("mnemonics: {:?}", mnemonics);
            todo!()
        }
        _ => todo!()
    }
}
