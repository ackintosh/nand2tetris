use std::path::{Path, PathBuf};
use std::io::{Error, BufWriter, Write};
use std::fs::File;
use crate::compilation_engine::CompilationEngine;
use crate::tokenizer::{Tokenizer, Token};

mod compilation_engine;
mod structures;
mod tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    assert_eq!(args.len(), 2, "Path to .jack file or a directory contains .jack file is required.");

    let path = Path::new(&args[1]);
    Analyzer::run(path);
}

struct Analyzer {}

impl Analyzer {
    fn run(source: &Path) {
        let jack_files = Self::jack_files(source).expect(format!("failed to read the path: {:?}", source).as_str());

        if jack_files.len() == 0 {
            panic!(".jack file is required.");
        }

        println!("jack_files: {:?}", jack_files);

        for jack_file in jack_files {
            let f = std::fs::File::open(jack_file.clone()).expect(format!("failed to open .jack file: {:?}", jack_file).as_str());
            let mut tokenizer = Tokenizer::new(f);
            let tokens = tokenizer.generate_tokens();

            let (destination, destination_token) = Self::source_to_destinations(&jack_file);
            println!("destination: {:?}", destination);
            println!("destination_token: {:?}", destination_token);

            save(destination_token, &tokens);

            let _class = CompilationEngine::compile(tokens);
        }
    }

    fn jack_files(path: &Path) -> Result<Vec<PathBuf>, Error>{
        if path.is_dir() {
            Ok(
                std::fs::read_dir(path)?.filter(|f| {
                    if !f.is_ok() {
                        return false;
                    }
                    "jack" == f.as_ref().unwrap().path().extension().expect("should have an extension")
                }).map(|f| {
                    f.unwrap().path()
                }).collect()
            )
        } else {
            if path.extension().expect("should have an extension") == "jack" {
                Ok(vec![path.to_path_buf()])
            } else {
                Ok(vec![])
            }
        }
    }

    fn source_to_destinations(path: &PathBuf) -> (PathBuf, PathBuf) {
        let mut  dest = path.clone();
        dest.set_extension("xml");

        let mut dest_token = dest.clone();
        dest_token = {
            let file_stem = {
                let file_stem = dest_token.file_stem().unwrap();
                file_stem.to_os_string()
            };
            dest_token.pop();
            dest_token.join(Path::new(format!("{}T.xml", file_stem.to_str().unwrap()).as_str()))
        };

        (dest, dest_token)
    }
}

trait Xml {
    fn xml(&self) -> String;
}

fn save<T>(destination: PathBuf, tokens: T) where T: Xml {
    let mut writer = BufWriter::new(
        File::create(destination).expect("failed to create a file")
    );
    writer.write_all(tokens.xml().as_bytes());
}

fn convert_to_xml_symbol(symbol: &str) -> String {
    match symbol {
        "<" => "&lt;".to_owned(),
        ">" => "&gt;".to_owned(),
        "&" => "&amp;".to_owned(),
        other => String::from(other),
    }
}
