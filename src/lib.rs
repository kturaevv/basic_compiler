pub mod ast;
pub mod emitter;
pub mod lexer;
pub mod parser;

use anyhow::Result;
use lexer::Lexer;
use parser::Parser;

pub struct Config {
    pub file_path: String,
    pub debug: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut min_arg_count = 1;

        let debug = args.contains(&String::from("--debug"));

        if debug {
            min_arg_count += 1;
        }

        if args.len() == min_arg_count {
            return Err("File path was not provided!");
        }
        if args.len() > min_arg_count + 1 {
            return Err("Too many arguments!");
        }

        let file_path = args[1].clone();
        Ok(Config { file_path, debug })
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
