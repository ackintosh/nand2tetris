use crate::parser::Parser;
use crate::code_writer::CodeWriter;

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

    let mut assembly_commands = vec![];
    let mut code_writer = CodeWriter{};

    loop {
        if let Some(command) = parser.advance() {
            println!("command: {:?}", command);
            assembly_commands.push(code_writer.code(command));
        } else {
            break;
        }
    }

    println!("assembly_commands: {:?}", assembly_commands);
}
