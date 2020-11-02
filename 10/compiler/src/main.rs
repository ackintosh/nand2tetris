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

struct Analyzer {
}

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
            let destination = Self::source_to_destination(&jack_file);
            println!("destination: {:?}", destination);
            save(destination, &tokens);

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

    fn source_to_destination(path: &PathBuf) -> PathBuf {
        let mut  dest = path.clone();
        dest.set_extension("xml");
        dest
    }
}

fn save(destination: PathBuf, tokens: &Vec<Token>) {
    let mut writer = BufWriter::new(File::create(destination).expect("failed to create a file"));
    writer.write_all(b"<tokens>\n");
    for token in tokens {
        match token {
            Token::Keyword(keyword) => writer.write_all(format!("<keyword>{}</keyword>\n", keyword).as_bytes()),
            Token::Symbol(symbol) => writer.write_all(format!("<symbol>{}</symbol>\n", convert_to_xml_symbol(&symbol)).as_bytes()),
            Token::Identifier(identifier) => writer.write_all(format!("<identifier>{}</identifier>\n", identifier).as_bytes()),
            Token::IntegerConst(integer_const) => writer.write_all(format!("<integerConstant>{}</integerConstant>\n", integer_const).as_bytes()),
            Token::StringConst(string_const) => writer.write_all(format!("<stringConstant>{}</stringConstant>\n", string_const).as_bytes()),
        };
    }
    writer.write_all(b"</tokens>");
}

fn convert_to_xml_symbol(symbol: &str) -> String {
    match symbol {
        "<" => "&lt;".to_owned(),
        ">" => "&gt;".to_owned(),
        "&" => "&amp;".to_owned(),
        other => String::from(other),
    }
}
