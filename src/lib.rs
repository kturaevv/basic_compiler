pub mod lexer;

use anyhow::Result;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 2 {
            return Err("Too many arguments!");
        }
        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<()> {
    lexer::Lexer::parse(config.file_path.as_str())?;
    Ok(())
}
