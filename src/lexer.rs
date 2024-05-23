use anyhow::Result;
use std::fs;

pub struct Lexer;

impl Lexer {
    pub fn parse(file_path: &str) -> Result<()> {
        let contents = fs::read_to_string(file_path)?;
        println!("{contents}");
        Ok(())
    }
}
