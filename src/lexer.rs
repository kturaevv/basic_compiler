use anyhow::Result;
use std::{iter::Peekable, str::Chars};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
enum Token {
    EOF,
    NEWLINE,
    IDENT,
    STRING(String),
    INTEGER(i64),
    FLOAT(f64),
    // Keywords
    PRINT,
    LABEL,
    GOTO,
    INPUT,
    LET,
    IF,
    THEN,
    ENDIF,
    WHILE,
    REPEAT,
    ENDWHILE,
    // Operators
    EQ,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EQEQ,
    NOTEQ,
    LT,
    LTEQ,
    GT,
    GTEQ,
}

impl Token {
    fn from_str(token: &str) -> Option<Token> {
        match token {
            "PRINT" => Some(Token::PRINT),
            "LABEL" => Some(Token::LABEL),
            "GOTO" => Some(Token::GOTO),
            "INPUT" => Some(Token::INPUT),
            "LET" => Some(Token::LET),
            "IF" => Some(Token::IF),
            "THEN" => Some(Token::THEN),
            "ENDIF" => Some(Token::ENDIF),
            "WHILE" => Some(Token::WHILE),
            "REPEAT" => Some(Token::REPEAT),
            "ENDWHILE" => Some(Token::ENDWHILE),
            "=" => Some(Token::EQ),
            "+" => Some(Token::PLUS),
            "-" => Some(Token::MINUS),
            "*" => Some(Token::ASTERISK),
            "/" => Some(Token::SLASH),
            "==" => Some(Token::EQEQ),
            "!=" => Some(Token::NOTEQ),
            "<" => Some(Token::LT),
            "<=" => Some(Token::LTEQ),
            ">" => Some(Token::GT),
            ">=" => Some(Token::GTEQ),
            _ => None,
        }
    }
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
                self.read_keyword(&mut contents);
            } else if current_character == '"' {
                self.read_string(&mut contents);
            } else if current_character.is_alphanumeric() {
                self.read_number(&mut contents);
            } else {
                contents.next();
            }
        }
        println!("{:?}", self.tokens);

        Ok(())
    }

    fn read_keyword(&mut self, contents: &mut Peekable<Chars>) {
        let mut token_string = String::new();

        while let Some(&current_character) = contents.peek() {
            contents.next();

            if current_character.is_whitespace() {
                break; // ERROR
            }
            token_string.push(current_character);

            let token = Token::from_str(token_string.as_str());
            if let Some(token) = token {
                self.tokens.push(token);
                return;
            }
        }

        let token = Token::from_str(token_string.as_str());
        if let Some(token) = token {
            self.tokens.push(token);
        } else {
            eprintln!("ERROR: Unknown token! {token_string}");
        }
    }

    fn read_string(&mut self, contents: &mut Peekable<Chars>) {
        contents.next(); // Skip opening quotes

        let mut token = String::new();
        while let Some(&current_character) = contents.peek() {
            contents.next();
            if current_character == '"' {
                break;
            }
            token.push(current_character);
        }
        self.tokens.push(Token::STRING(token));
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
            true => self
                .tokens
                .push(Token::FLOAT(tok.parse().unwrap_or_else(|err| {
                    println!("Error parsing float: {err}");
                    1.0 // Default value in case of an error
                }))),
            false => self
                .tokens
                .push(Token::INTEGER(tok.parse().unwrap_or_else(|err| {
                    println!("Error parsing integer: {err}");
                    1
                }))),
        };
    }
}
