use crate::parser::Parser;
use crate::code_writer::CodeWriter;
use std::path::PathBuf;
use std::io::{BufWriter, Write};
use std::fs::File;

mod code_writer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    assert_eq!(args.len(), 2, "A path to .vm file or directory contains .vm file is required");

    let path = std::path::Path::new(&args[1]);
    if path.is_dir() {
        todo!("directory is not supported for now");
    }

    let mut parser = Parser::new(
        std::fs::File::open(path).expect("failed to open the file")
    );

    let mut assembly_codes = vec![];
    let mut code_writer = CodeWriter::new();

    loop {
        if let Some(command) = parser.advance() {
            println!("command: {:?}", command);
            assembly_codes.append(&mut code_writer.code(command));
        } else {
            break;
        }
    }

    println!("assembly_codes: {:?}", assembly_codes);

    let mut output_path = PathBuf::from(path);
    output_path.set_extension("asm");

    let mut writer = BufWriter::new(File::create(output_path).expect("failed to create asm file"));
    for code in assembly_codes.iter() {
        writer.write_all(code.as_bytes()).expect("failed to write assembly codes");
        writer.write_all(b"\n").unwrap();
    }

}
