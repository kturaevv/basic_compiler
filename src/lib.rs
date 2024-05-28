pub mod lexer;
pub mod parser;

use std::fs;

use anyhow::Result;
use lexer::Lexer;
use parser::{Emitter, Parser};

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 1 {
            return Err("File path was not provided!");
        }
        if args.len() > 2 {
            return Err("Too many arguments!");
        }
        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<()> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut lexer = Lexer::new();
    lexer.parse(contents.as_str())?;

    let mut emitter = Emitter::new("./out.c");

    let mut parser = Parser::new();
    parser.check(&lexer, &mut emitter)?;

    emitter.write_to_file();

    Ok(())
}
