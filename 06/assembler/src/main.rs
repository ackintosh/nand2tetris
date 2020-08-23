use std::ffi::OsStr;
use crate::parser::Parser;
use std::path::PathBuf;
use std::io::{BufWriter, Write};
use std::fs::File;

mod code;
mod parser;
mod symbol_table;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);
    assert_eq!(args.len(), 2, "the path to .asm file is required");

    let path = std::path::Path::new(&args[1]);
    assert_eq!(
        path.extension().and_then(OsStr::to_str).expect("expects .asm file"),
        "asm",
        ".asm file is required: {}", path.display()
    );

    let mut parser = Parser::new(
        std::fs::File::open(path).expect("file not found")
    );

    parser.scan_labels();

    let mut binary_code: Vec<[bool; 16]> = vec![];

    loop {
        if let Some(command) = parser.advance() {
            println!("command: {:?}", command);
            let bits = code::code(command);
            println!("bits: {:?}", bits);
            binary_code.push(bits);
        } else {
            break;
        }
    }

    let mut output_path = PathBuf::from(path);
    output_path.set_extension("hack");

    let mut writer = BufWriter::new(File::create(output_path).expect("failed to create hack file"));
    for line in binary_code.iter() {
        let mut code: String = line.iter()
            .map(|b| if *b { "1" } else { "0" })
            .collect::<Vec<&str>>()
            .join("");
        code.push_str("\n");
        writer.write_all(code.as_bytes()).expect("failed to write the binary code");
    }
}
