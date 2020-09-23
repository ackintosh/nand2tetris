use crate::parser::Parser;
use crate::code_writer::CodeWriter;
use std::path::{PathBuf, Path};
use std::io::{BufWriter, Write, Error};
use std::fs::File;

mod code_writer;
mod parser;

// TODO: ブートストラップコードの実装
// TODO: ラベルジェネレータの一意性修正

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    assert_eq!(args.len(), 2, "A path to .vm file or directory contains .vm file is required");

    let path = std::path::Path::new(&args[1]);

    let vm_files = vm_files(path).expect("failed to open files");
    if vm_files.len() == 0 {
        panic!(format!("{}: should have vm files", &args[1]))
    }
    println!("{:?}", vm_files);

    let mut assembly_codes = vec![];
    for pathbuf in vm_files.iter() {
        assembly_codes.extend(parse(pathbuf.as_path()));
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

fn vm_files(path: &Path) -> Result<Vec<PathBuf>, Error> {
    if path.is_dir() {
        Ok(
            std::fs::read_dir(path)?.filter(|f| {
                if !f.is_ok() {
                    return false;
                }
                "vm" == f.as_ref().unwrap().path().extension().expect("should have an extension")
            }).map(|e| {
                e.unwrap().path()
            }).collect()
        )
    } else {
        if "vm" == path.extension().expect("should have an extension") {
            Ok(vec![path.to_path_buf()])
        } else {
            Ok(vec![])
        }
    }
}

fn parse(path: &Path) -> Vec<String> {
    let mut parser = Parser::new(
        std::fs::File::open(path).expect("failed to open the file")
    );

    let mut assembly_codes = vec![];
    let mut code_writer = CodeWriter::new(String::from(path.file_stem().unwrap().to_str().unwrap()));

    loop {
        if let Some(command) = parser.advance() {
            assembly_codes.append(&mut code_writer.code(command));
        } else {
            break;
        }
    }

    assembly_codes
}
