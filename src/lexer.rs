use anyhow::Result;
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
enum Keywords {
    Print,
}

#[derive(Debug)]
enum Token {
    Command(Keywords),
    String(String),
    Integer(i64),
    Float(f64),
}

#[derive(Default)]
pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            ..Default::default()
        }
    }

    pub fn parse(&mut self, contents: &str) -> Result<()> {
        let mut contents = contents.chars().peekable();

        while let Some(&current_character) = contents.peek() {
            if current_character.is_alphabetic() {
                self.read_string(&mut contents);
            } else if current_character.is_alphanumeric() {
                self.read_number(&mut contents);
            }
            contents.next();
        }
        println!("{:?}", self.tokens);

        Ok(())
    }

    fn read_string(&mut self, contents: &mut Peekable<Chars>) {
        let mut tok = String::new();
        while let Some(&current_character) = contents.peek() {
            if current_character == '"' {
                break;
            }
            tok.push(current_character);
            contents.next();
        }

        if tok.trim().chars().all(|c| c.is_uppercase()) {
            self.tokens.push(Token::Command(Keywords::Print))
        } else {
            self.tokens.push(Token::String(tok));
        }
    }

    fn read_number(&mut self, contents: &mut Peekable<Chars>) {
        let mut is_float = false;
        let mut tok = String::new();

        while let Some(&current_character) = contents.peek() {
            if current_character.is_whitespace() {
                break;
            }

            if current_character == '.' {
                is_float = true;
            }

            tok.push(current_character);
            contents.next();
        }

        match is_float {
            true => self.tokens.push(Token::Float(tok.parse().unwrap())),
            false => self.tokens.push(Token::Integer(tok.parse().unwrap())),
        };
    }
}
