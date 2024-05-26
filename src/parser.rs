use crate::lexer::{Lexer, Token};
use anyhow::{anyhow, Ok, Result};
use std::iter::Peekable;

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            ..Default::default()
        }
    }

    // program ::= {statement}
    pub fn check(&mut self, lexer: &Lexer) -> Result<()> {
        let mut tokens = lexer.tokens.iter().peekable();

        while tokens.peek().is_some() {
            println!("--- {} ---", tokens.peek().unwrap());
            self.statement(&mut tokens)?;
        }

        Ok(())
    }

    // statement ::= PRINT (expression | string) nl
    //               IF comparison "THEN" nl {statement} "ENDIF" nl
    //               WHILE comparison "REPEAT" nl {statement} "ENDWHILE" nl
    //               LABEL ident nl
    //               GOTO ident nl
    //               LET ident "=" expression nl
    //               INPUT ident nl
    fn match_variable<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::VARIABLE(value)) => println!("IDENT ({value})"),
            _ => Err(anyhow!("Invalid variable!"))?,
        }

        Ok(())
    }

    fn nl<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::NEWLINE) => (),
            val => Err(anyhow!("Should be followed by NEWLINE: {:?}", val))?,
        }
        Ok(())
    }

    fn statement<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::NEWLINE) => {
                println!("NEWLINE");
            }
            Some(Token::PRINT) => {
                println!("STATEMENT: PRINT");
                match tokens.peek() {
                    Some(Token::STRING(value)) => {
                        println!("STRING: {value}");
                        tokens.next();
                    }
                    _ => self.expression(tokens)?,
                }
            }
            Some(Token::IF) => {
                println!("STATEMENT: IF");

                self.comparison(tokens)?;

                match tokens.next() {
                    Some(Token::THEN) => println!("STATEMENT: THEN"),
                    val => return Err(anyhow!("IF should be followed by THEN, got {:?}", val)),
                }

                self.nl(tokens)?;
                self.statement(tokens)?;

                match tokens.next() {
                    Some(Token::ENDIF) => println!("STATEMENT: ENDIF"),
                    val => return Err(anyhow!("IF should be followed by ENDIF: Actual {:?}", val)),
                }
            }
            Some(Token::WHILE) => {
                println!("STATEMENT: WHILE");

                self.comparison(tokens)?;

                match tokens.next() {
                    Some(Token::REPEAT) => println!("STATEMENT: REPEAT"),
                    val => {
                        return Err(anyhow!("WHILE should be followed by REPEAT, got {:?}", val))
                    }
                }

                self.nl(tokens)?;
                self.statement(tokens)?;

                match tokens.next() {
                    Some(Token::ENDWHILE) => println!("STATEMENT: ENDWHILE"),
                    val => {
                        return Err(anyhow!(
                            "WHILE should be followed by ENDWHILE, got {:?}",
                            val
                        ))
                    }
                }
            }
            Some(Token::LABEL) => {
                println!("STATEMENT: LABEL");
                self.match_variable(tokens)?;
            }
            Some(Token::GOTO) => {
                println!("STATEMENT: GOTO");
                self.match_variable(tokens)?;
            }
            Some(Token::LET) => {
                println!("STATEMENT: LET");
                self.match_variable(tokens)?;

                match tokens.next() {
                    Some(Token::EQ) => println!("STATEMENT: EQ"),
                    val => return Err(anyhow!("LET should be followed by '=', got {:?}", val)),
                }

                self.expression(tokens)?;
            }
            Some(Token::INPUT) => {
                println!("STATEMENT: INPUT");
                self.match_variable(tokens)?;
            }
            Some(token) => Err(anyhow!("Invalid statement at: {token}"))?,
            None => panic!("None encountered!!!"),
        }
        self.nl(tokens)?;
        Ok(())
    }

    fn is_comparison<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::EQ) => (),
            Some(Token::PLUS) => (),
            Some(Token::MINUS) => (),
            Some(Token::ASTERISK) => (),
            Some(Token::SLASH) => (),
            Some(Token::EQEQ) => (),
            Some(Token::NOTEQ) => (),
            Some(Token::LT) => (),
            Some(Token::LTEQ) => (),
            Some(Token::GT) => (),
            Some(Token::GTEQ) => (),
            _ => Err(anyhow!("Expected comparison!"))?,
        }
        Ok(())
    }

    // comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
    fn comparison<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        self.expression(tokens)?;

        println!("COMPARISON");
        self.is_comparison(tokens)?;

        while self.is_comparison(tokens).is_ok() {
            self.expression(tokens)?;
        }
        Ok(())
    }

    // expression ::= term {( "-" | "+" ) term}
    fn expression<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("EXPRESSION");

        self.term(tokens)?;

        while let Some(token) = tokens.peek() {
            match token {
                Token::PLUS => {
                    println!("UNARY PLUS");
                    tokens.next();
                    self.term(tokens)?;
                }
                Token::MINUS => {
                    println!("UNARY MINUS");
                    tokens.next();
                    self.term(tokens)?;
                }
                _ => break,
            }
        }

        Ok(())
    }

    // term ::= unary {( "/" | "*" ) unary}
    fn term<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("TERM");

        self.unary(tokens)?;

        while let Some(token) = tokens.peek() {
            match token {
                Token::SLASH => {
                    println!("TERM SLASH");
                    tokens.next();
                    self.unary(tokens)?;
                }
                Token::ASTERISK => {
                    println!("TERM ASTERISK");
                    tokens.next();
                    self.unary(tokens)?;
                }
                _ => break,
            }
        }

        Ok(())
    }

    // unary ::= ["+" | "-"] primary
    fn unary<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("UNARY");

        // optional unary
        match tokens.peek() {
            Some(Token::PLUS) => {
                tokens.next();
                println!("UNARY PLUS")
            }
            Some(Token::MINUS) => {
                tokens.next();
                println!("UNARY MINUS");
            }
            _ => (),
        }

        self.primary(tokens)?;
        Ok(())
    }

    // primary ::= number | ident
    fn primary<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::INTEGER(value)) => println!("PRIMARY ({value})"),
            Some(Token::FLOAT(value)) => println!("PRIMARY ({value})"),
            Some(Token::VARIABLE(value)) => println!("IDENT ({value})"),
            Some(token) => Err(anyhow!("Unexpected token! {token}"))?,
            None => Err(anyhow!("Unexpected token! None"))?,
        }
        Ok(())
    }
}
