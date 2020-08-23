// 6.3.2 Codeモジュール
// Hackアセンブリ言語のニーモニックをバイナリコードへ変換する

use crate::parser::Command;

pub fn code(command: Command) -> [bool; 16] {
    match command {
        // A命令
        // 0vvvv vvvv vvvv vvvv
        Command::ACommand(s) => {
            // A命令のvalue部分を、2進数の文字列に変換
            let bit_string = format!("{:b}", s.parse::<u16>().unwrap());

            let mut bit_array = [false; 16];
            let mut count = 15;
            for b in bit_string.chars().rev() {
                if b == '1' {
                    bit_array[count] = true
                } else {
                    bit_array[count] = false
                }
                count -= 1;
            }

            bit_array
        }
        // C命令
        // 111a cccc ccdd djjj
        Command::CCommand(mnemonics) => {
            println!("mnemonics: {:?}", mnemonics);

            let mut bits: Vec<bool> = vec![true, true, true];
            bits.append(&mut a_comp(mnemonics.1.as_str()).to_vec());
            bits.append(&mut dest(mnemonics.0.as_str()).to_vec());
            bits.append(&mut jump(mnemonics.2.as_str()).to_vec());

            assert_eq!(bits.len(), 16);

            let mut bit_array = [false; 16];
            for (i, b) in bits.iter().enumerate() {
                bit_array[i] = *b;
            }

            bit_array
        }
        _ => todo!()
    }
}

// a + comp を変換したバイナリコード(計7bit)を返す
// P.119
fn a_comp(comp: &str) -> [bool; 7] {
    match comp {
        // a = 0
        "0" => [
            false,
            true, false, true, false, true, false
        ],
        "1" => [
            false,
            true, true, true, true, true, true
        ],
        "-1" => [
            false,
            true, true, true, false, true, false
        ],
        "D" => [
            false,
            false, false, true, true, false, false
        ],
        "A" => [
            false,
            true, true, false, false, false, false
        ],
        "!D" => [
            false,
            false, false, true, true, false, true
        ],
        "!A" => [
            false,
            true, true, false, false, false, true
        ],
        "-D" => [
            false,
            false, false, true, true, true, true
        ],
        "-A" => [
            false,
            true, true, false, false, true, true
        ],
        "D+1" => [
            false,
            false, true, true, true, true, true
        ],
        "A+1" => [
            false,
            true, true, false, true, true, true
        ],
        "D-1" => [
            false,
            false, false, true, true, true, false
        ],
        "A-1" => [
            false,
            true, true, false, false, true, false
        ],
        "D+A" => [
            false,
            false, false, false, false, true, false
        ],
        "D-A" => [
            false,
            false, true, false, false, true, true
        ],
        "A-D" => [
            false,
            false, false, false, true, true, true
        ],
        "D&A" => [
            false,
            false, false, false, false, false, false
        ],
        "D|A" => [
            false,
            false, true, false, true, false, true
        ],
        // a = 1
        "M" => [
            true,
            true, true, false, false, false, false
        ],
        "!M" => [
            true,
            true, true, false, false, false, true
        ],
        "-M" => [
            true,
            true, true, false, false, true, true
        ],
        "M+1" => [
            true,
            true, true, false, true, true, true
        ],
        "M-1" => [
            true,
            true, true, false, false, true, false
        ],
        "D+M" => [
            true,
            false, false, false, false, true, false
        ],
        "D-M" => [
            true,
            false, true, false, false, true, true
        ],
        "M-D" => [
            true,
            false, false, false, true, true, true
        ],
        "D&M" => [
            true,
            false, false, false, false, false, false
        ],
        "D|M" => [
            true,
            false, true, false, true, false, true
        ],
        _ => todo!("comp: {}", comp)
    }
}

// dest を変換したバイナリコード(3bit)を返す
// P.119
fn dest(dest: &str) -> [bool; 3] {
    match dest {
        "null" => [false, false, false],
        "M" => [false, false, true],
        "D" => [false, true, false],
        "MD" => [false, true, true],
        "A" => [true, false, false],
        "AM" => [true, false, true],
        "AD" => [true, true, false],
        "AMD" => [true, true, true],
        _ => todo!("dest: {}", dest)
    }
}

// jump を変換したバイナリコード(3bit)を返す
// P.119
fn jump(jump: &str) -> [bool; 3] {
    match jump {
        "null" => [false, false, false],
        "JGT" => [false, false, true],
        "JEQ" => [false, true, false],
        "JGE" => [false, true, true],
        "JLT" => [true, false, false],
        "JNE" => [true, false, true],
        "JLE" => [true, true, false],
        "JMP" => [true, true, true],
        _ => todo!("jump: {}", jump)
    }
}
