use anyhow::Result;
use std::{iter::Peekable, str::Chars};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Token {
    EOF,
    VARIABLE,
    NEWLINE,
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
            "<" => Some(Token::LT),
            ">" => Some(Token::GT),
            "==" => Some(Token::EQEQ),
            "<=" => Some(Token::LTEQ),
            ">=" => Some(Token::GTEQ),
            "!=" => Some(Token::NOTEQ),
            "\n" => Some(Token::NEWLINE),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            ..Default::default()
        }
    }

    pub fn parse(&mut self, contents: &str) -> Result<()> {
        let mut contents = contents.chars().peekable();

        while let Some(current_character) = contents.next() {
            // Skip whitespace
            if current_character.is_whitespace() {
                if current_character == '\n' {
                    self.tokens.push(Token::NEWLINE);
                }
                continue;
            }

            // Skip comments
            if current_character == '#' {
                while let Some(&ch) = contents.peek() {
                    if ch == '\n' {
                        break;
                    }
                    contents.next();
                }
                continue;
            }

            // Handle single and multi-character tokens
            let mut current_token = String::new();
            if current_character != '"' {
                current_token.push(current_character);
            }

            // Handle 2-char tokens
            if let Some(next_char) = contents.peek() {
                current_token.push(*next_char);
                if let Some(multi_char_token) = Token::from_str(&current_token) {
                    self.tokens.push(multi_char_token);
                    contents.next(); // Consume the peeked character
                    continue;
                } else {
                    current_token.pop();
                }
            }

            // Handle 1-char token
            if let Some(single_char_token) = Token::from_str(&current_token) {
                self.tokens.push(single_char_token);
                continue;
            }

            // Handle keywords and literals
            if current_character.is_alphabetic() {
                self.read_keyword(&mut contents, &mut current_token);
            } else if current_character == '"' {
                self.read_string(&mut contents, &mut current_token);
            } else if current_character.is_numeric() {
                self.read_number(&mut contents, &mut current_token);
            }
        }

        Ok(())
    }

    fn read_keyword(&mut self, contents: &mut Peekable<Chars>, token: &mut String) {
        while let Some(&current_character) = contents.peek() {
            if current_character.is_whitespace() {
                break;
            }
            token.push(current_character);
            contents.next();
        }

        if let Some(keyword_token) = Token::from_str(token.as_str()) {
            self.tokens.push(keyword_token);
        }
    }

    fn read_string(&mut self, contents: &mut Peekable<Chars>, token: &mut String) {
        while let Some(&current_character) = contents.peek() {
            if current_character == '"' {
                contents.next();
                break;
            }
            token.push(current_character);
            contents.next();
        }
        self.tokens.push(Token::STRING(token.clone()));
    }

    fn read_number(&mut self, contents: &mut Peekable<Chars>, token: &mut String) {
        let mut is_float = false;

        while let Some(&current_character) = contents.peek() {
            if current_character.is_whitespace() {
                break;
            }

            if current_character == '.' {
                is_float = true;
            }

            token.push(current_character);
            contents.next();
        }

        if is_float {
            self.tokens.push(Token::FLOAT(token.parse().unwrap_or(1.0)));
        } else {
            self.tokens.push(Token::INTEGER(token.parse().unwrap_or(1)));
        }
    }
}
