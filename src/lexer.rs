use anyhow::{anyhow, Result};
use std::{iter::Peekable, str::Chars};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum Token {
    EOF,
    NEWLINE,
    VARIABLE(String),
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
        Some(match token {
            "PRINT" => Token::PRINT,
            "LABEL" => Token::LABEL,
            "GOTO" => Token::GOTO,
            "INPUT" => Token::INPUT,
            "LET" => Token::LET,
            "IF" => Token::IF,
            "THEN" => Token::THEN,
            "ENDIF" => Token::ENDIF,
            "WHILE" => Token::WHILE,
            "REPEAT" => Token::REPEAT,
            "ENDWHILE" => Token::ENDWHILE,
            "=" => Token::EQ,
            "+" => Token::PLUS,
            "-" => Token::MINUS,
            "*" => Token::ASTERISK,
            "/" => Token::SLASH,
            "<" => Token::LT,
            ">" => Token::GT,
            "==" => Token::EQEQ,
            "<=" => Token::LTEQ,
            ">=" => Token::GTEQ,
            "!=" => Token::NOTEQ,
            "\n" => Token::NEWLINE,
            _ => return None,
        })
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

        while let Some(cur_char) = contents.next() {
            let mut current_token = String::new();

            match cur_char {
                '\n' => self.tokens.push(Token::NEWLINE),
                '#' => self.skip_comments(&mut contents),
                '"' => self.read_string(&mut contents, &mut current_token),
                '=' | '+' | '-' | '*' | '/' | '<' | '>' | '!' => {
                    self.read_short_keyword(&mut contents, &mut current_token, &cur_char)
                }
                _ if cur_char.is_whitespace() => continue,
                _ if cur_char.is_alphabetic() => {
                    self.read_keyword(&mut contents, &mut current_token, &cur_char)
                }
                _ if cur_char.is_numeric() => {
                    self.read_number(&mut contents, &mut current_token, &cur_char)
                }
                _ => Err(anyhow!("Unknown token: {}", cur_char))?,
            }
        }

        Ok(())
    }

    fn is_keyword(&self, character: &char) -> bool {
        Token::from_str(character.to_string().as_str()).is_some()
    }

    fn next_is_keyword(&self, contents: &mut Peekable<Chars>) -> bool {
        if let Some(next_char) = contents.peek() {
            return self.is_keyword(next_char);
        }
        false
    }

    fn skip_comments(&mut self, contents: &mut Peekable<Chars>) {
        contents.find(|&ch| ch == '\n');
    }

    fn read_short_keyword(
        &mut self,
        contents: &mut Peekable<Chars>,
        token: &mut String,
        current_character: &char,
    ) {
        if token.is_empty() {
            token.push(*current_character)
        }

        // Handle 2-char tokens
        if let Some(next_char) = contents.peek() {
            token.push(*next_char);
            if let Some(multi_char_token) = Token::from_str(token) {
                self.tokens.push(multi_char_token);
                contents.next(); // Consume the peeked character
                return;
            } else {
                token.pop();
            }
        }

        // Handle 1-char token
        if let Some(single_char_token) = Token::from_str(token) {
            self.tokens.push(single_char_token);
        }
    }

    fn read_keyword(
        &mut self,
        contents: &mut Peekable<Chars>,
        token: &mut String,
        current_character: &char,
    ) {
        token.push(*current_character);

        while let Some(current_character) = contents.next() {
            if current_character.is_whitespace() {
                break;
            }
            token.push(current_character);

            if self.next_is_keyword(contents) {
                break;
            }
        }

        if let Some(keyword_token) = Token::from_str(token.as_str()) {
            self.tokens.push(keyword_token);
        } else {
            self.tokens.push(Token::VARIABLE(token.clone()));
        }
    }

    fn read_number(
        &mut self,
        contents: &mut Peekable<Chars>,
        token: &mut String,
        current_character: &char,
    ) {
        token.push(*current_character);

        let mut is_float = false;

        while let Some(current_character) = contents.next() {
            if current_character.is_whitespace() {
                break;
            }

            if current_character == '.' {
                is_float = true;
            }

            token.push(current_character);

            if self.next_is_keyword(contents) {
                break;
            }
        }

        if is_float {
            match token.parse::<f64>() {
                Ok(token) => self.tokens.push(Token::FLOAT(token)),
                Err(_) => println!("Failed to parse TOKEN::FLOAT {token}"),
            }
        } else {
            match token.parse::<i64>() {
                Ok(token) => self.tokens.push(Token::INTEGER(token)),
                Err(_) => println!("Failed to parse TOKEN::INTEGER {token}"),
            }
        }
    }

    fn read_string(&mut self, contents: &mut Peekable<Chars>, token: &mut String) {
        token.extend(contents.take_while(|&c| c != '"'));
        self.tokens.push(Token::STRING(token.clone()));
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TOKEN::{:?}", self)
    }
}
