use crate::lexer::{Lexer, Token};
use anyhow::{anyhow, Ok, Result};
use std::collections::HashSet;
use std::iter::Peekable;

#[derive(Default)]
pub struct Parser {
    variables: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
}

#[derive(Default)]
pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
}

impl Emitter {
    pub fn new(full_path: &str) -> Emitter {
        Emitter {
            full_path: full_path.to_string(),
            ..Default::default()
        }
    }

    pub fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    pub fn emit_line(&mut self, code: &str) {
        self.emit(code);
        self.code.push('\n');
    }

    pub fn emit_header(&mut self, header: &str) {
        self.header.push_str(header);
        self.header.push('\n');
    }

    pub fn write_to_file(&self) {
        let content = [self.header.as_bytes(), self.code.as_bytes()].concat();
        std::fs::write(self.full_path.as_str(), content).expect("Couldnt write to file");
    }
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            ..Default::default()
        }
    }

    // program ::= {statement}
    pub fn check(&mut self, lexer: &Lexer, emitter: &mut Emitter) -> Result<()> {
        emitter.emit_header("#include <stdio.h>");
        emitter.emit_header("int main(void){");

        let mut tokens = lexer.tokens.iter().peekable();

        while tokens.peek().is_some() {
            self.statement(&mut tokens)?;
        }

        emitter.emit_line("return 0;");
        emitter.emit_line("}");

        for label in &self.labels_gotoed {
            if self.labels_declared.contains(label) {
                return Err(anyhow!("Attemt to GOTO to undeclared label! {label}"));
            }
        }
        Ok(())
    }

    // statement ::= PRINT (expression | string) nl
    //               IF comparison "THEN" nl {statement} "ENDIF" nl
    //               WHILE comparison "REPEAT" nl {statement} "ENDWHILE" nl
    //               LABEL var nl
    //               GOTO var nl
    //               LET var "=" expression nl
    //               INPUT var nl
    fn statement<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::NEWLINE) => println!("NEWLINE"),
            Some(Token::PRINT) => self.statement_print(tokens)?,
            Some(Token::IF) => self.statement_if(tokens)?,
            Some(Token::WHILE) => self.statement_while(tokens)?,
            Some(Token::LABEL) => self.statement_label(tokens)?,
            Some(Token::GOTO) => self.statement_goto(tokens)?,
            Some(Token::LET) => self.statement_let(tokens)?,
            Some(Token::INPUT) => self.statement_input(tokens)?,
            Some(token) => Err(anyhow!("Invalid statement at: {token}"))?,
            None => panic!("None encountered!!!"),
        }
        Ok(())
    }

    fn var<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<String>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::VARIABLE(value)) => {
                println!("VAR ({value})");
                Ok(value.clone())
            }
            _ => Err(anyhow!("Invalid variable!"))?,
        }
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

    // statement ::= PRINT (expression | string) nl
    fn statement_print<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("STATEMENT: PRINT");
        match tokens.peek() {
            Some(Token::STRING(value)) => {
                println!("STRING: {value}");
                tokens.next();
            }
            _ => self.expression(tokens)?,
        }
        self.nl(tokens)?;
        Ok(())
    }

    // statement ::= IF comparison "THEN" nl {statement} "ENDIF" nl
    fn statement_if<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("STATEMENT: IF");

        self.comparison(tokens)?;

        match tokens.next() {
            Some(Token::THEN) => println!("STATEMENT: THEN"),
            val => return Err(anyhow!("IF should be followed by THEN, got {:?}", val)),
        }

        self.nl(tokens)?;

        while let Some(token) = tokens.peek() {
            match token {
                Token::ENDIF => {
                    println!("STATEMENT: ENDIF");
                    tokens.next();
                    self.nl(tokens)?;
                    return Ok(());
                }
                _ => self.statement(tokens)?,
            }
        }
        Err(anyhow!("IF should be followed by ENDIF"))
    }

    // statement ::= WHILE comparison "REPEAT" nl {statement} "ENDWHILE" nl
    fn statement_while<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("STATEMENT: WHILE");

        self.comparison(tokens)?;

        match tokens.next() {
            Some(Token::REPEAT) => println!("STATEMENT: REPEAT"),
            val => return Err(anyhow!("WHILE should be followed by REPEAT, got {:?}", val)),
        }

        self.nl(tokens)?;

        while let Some(token) = tokens.peek() {
            match token {
                Token::ENDWHILE => {
                    println!("STATEMENT: ENDWHILE");
                    tokens.next();
                    self.nl(tokens)?;
                    return Ok(());
                }
                _ => self.statement(tokens)?,
            }
        }
        Err(anyhow!("WHILE should be followed by ENDWHILE",))
    }

    // statement ::= LABEL var nl
    fn statement_label<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("STATEMENT: LABEL");
        let var = self.var(tokens)?;

        if !self.labels_declared.insert(var.clone()) {
            return Err(anyhow!("Label aready exists! {var}"));
        }

        self.nl(tokens)?;
        Ok(())
    }

    // statement ::= GOTO var nl
    fn statement_goto<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("STATEMENT: GOTO");

        let var = self.var(tokens)?;

        self.labels_gotoed.insert(var.clone());

        self.nl(tokens)?;

        Ok(())
    }

    // statement ::= LET var "=" expression nl
    fn statement_let<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("STATEMENT: LET");

        let var = self.var(tokens)?;

        self.variables.insert(var);

        match tokens.next() {
            Some(Token::EQ) => println!("STATEMENT: EQ"),
            val => return Err(anyhow!("LET should be followed by '=', got {:?}", val)),
        }

        self.expression(tokens)?;
        self.nl(tokens)?;
        Ok(())
    }

    // statement ::= INPUT var nl
    fn statement_input<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        println!("STATEMENT: INPUT");
        let var = self.var(tokens)?;

        self.variables.insert(var);

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

    // primary ::= number | var
    fn primary<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::INTEGER(value)) => println!("PRIMARY ({value})"),
            Some(Token::FLOAT(value)) => println!("PRIMARY ({value})"),
            Some(Token::VARIABLE(value)) => {
                if !self.variables.contains(value) {
                    Err(anyhow!("Variable referenced before assignment!"))?
                }
                println!("var ({value})");
            }
            Some(token) => Err(anyhow!("Unexpected token! {token}"))?,
            None => Err(anyhow!("Unexpected token! None"))?,
        }
        Ok(())
    }
}
