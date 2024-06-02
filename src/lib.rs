pub mod ast;
pub mod emitter;
pub mod lexer;
pub mod parser;

use anyhow::Result;
use lexer::Lexer;
use parser::Parser;

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
    let contents = std::fs::read_to_string(config.file_path)?;

    let mut lexer = Lexer::new();
    lexer.parse(contents.as_str())?;

    let mut parser = Parser::new();
    parser.check(&lexer)?;

    Ok(())
}
