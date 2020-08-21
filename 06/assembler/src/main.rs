use std::ffi::OsStr;
use crate::parser::Parser;

mod parser;

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

    println!("{:?}", parser.advance());
    println!("{:?}", parser.advance());
    println!("{:?}", parser.advance());
}
