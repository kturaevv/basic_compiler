use crate::ast;
use crate::lexer::{Lexer, Token};
use anyhow::{anyhow, Ok, Result};
use std::collections::HashSet;
use std::iter::Peekable;
use std::ops::Mul;

#[derive(Default)]
pub struct Parser {
    pub ast: Vec<Token>,
    variables: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
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
            self.statement(&mut tokens)?;
        }

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
            Some(Token::NEWLINE) => (),
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
            Some(value @ Token::VARIABLE(content)) => {
                self.ast.push(value.clone());
                Ok(content.clone())
            }
            _ => Err(anyhow!("Invalid variable!"))?,
        }
    }

    fn nl<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::NEWLINE) => self.ast.push(Token::NEWLINE),
            val => Err(anyhow!("Should be followed by NEWLINE: {:?}", val))?,
        }
        Ok(())
    }

    // statement ::= PRINT (expression | string) nl
    fn statement_print<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        self.ast.push(Token::PRINT);

        match tokens.peek() {
            Some(Token::STRING(_)) => {
                self.ast.push(tokens.next().unwrap().clone());
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
        self.ast.push(Token::IF);

        self.comparison(tokens)?;

        match tokens.next() {
            Some(Token::THEN) => self.ast.push(Token::THEN),
            val => return Err(anyhow!("IF should be followed by THEN, got {:?}", val)),
        }

        self.nl(tokens)?;

        while let Some(token) = tokens.peek() {
            match token {
                Token::ENDIF => {
                    self.ast.push(tokens.next().unwrap().clone());
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
        self.ast.push(Token::WHILE);

        self.comparison(tokens)?;

        match tokens.next() {
            Some(Token::REPEAT) => self.ast.push(Token::REPEAT),
            val => return Err(anyhow!("WHILE should be followed by REPEAT, got {:?}", val)),
        }

        self.nl(tokens)?;

        while let Some(token) = tokens.peek() {
            match token {
                Token::ENDWHILE => {
                    self.ast.push(tokens.next().unwrap().clone());
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
        self.ast.push(Token::LABEL);

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
        self.ast.push(Token::GOTO);

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
        self.ast.push(Token::LET);

        let var = self.var(tokens)?;

        self.variables.insert(var);

        match tokens.next() {
            Some(value @ Token::EQ) => self.ast.push(value.clone()),
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
        self.ast.push(Token::INPUT);

        let var = self.var(tokens)?;

        self.variables.insert(var);

        self.nl(tokens)?;
        Ok(())
    }

    fn is_comparison<'a, I>(&mut self, tokens: &mut Peekable<I>) -> bool
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.peek() {
            Some(Token::EQEQ) | Some(Token::NOTEQ) | Some(Token::LT) | Some(Token::LTEQ)
            | Some(Token::GT) | Some(Token::GTEQ) => {
                self.ast.push(tokens.next().unwrap().clone());
                true
            }
            _ => false,
        }
    }

    // comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
    fn comparison<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        self.expression(tokens)?;
        self.is_comparison(tokens);
        self.expression(tokens)?;

        while self.is_comparison(tokens) {
            self.expression(tokens)?;
        }
        Ok(())
    }

    // expression ::= term {( "-" | "+" ) term}
    fn expression<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        self.term(tokens)?;

        while let Some(token) = tokens.peek() {
            match token {
                Token::PLUS | Token::MINUS => {
                    self.ast.push(tokens.next().unwrap().clone());
                    self.term(tokens)?;
                }
                _ => break,
            }
        }

        Ok(())
    }

    // term ::= unary {( "/" | "*" ) unary}
    fn term<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Term>
    where
        I: Iterator<Item = &'a Token>,
    {
        let unary = self.unary(tokens)?;

        if let Some(token) = tokens.peek() {
            match token {
                Token::ASTERISK => Ok(ast::Term::Mul(Box::new(self.term(tokens)?))),
                Token::SLASH => Ok(ast::Term::Div(Box::new(self.term(tokens)?))),
                _ => Ok(ast::Term::Unary(unary)),
            }
        } else {
            Ok(ast::Term::Unary(unary))
        }
    }

    // unary ::= ["+" | "-"] primary
    fn unary<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Unary>
    where
        I: Iterator<Item = &'a Token>,
    {
        // optional unary
        return match tokens.peek() {
            Some(Token::PLUS) => Ok(ast::Unary::Positive(Box::new(self.unary(tokens)?))),
            Some(Token::MINUS) => Ok(ast::Unary::Negative(Box::new(self.unary(tokens)?))),
            _ => Ok(ast::Unary::Primary(self.primary(tokens)?)),
        };
    }

    // primary ::= number | var
    fn primary<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Primary>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::INTEGER(val)) => Ok(ast::Primary::Integer(*val)),
            Some(Token::FLOAT(val)) => Ok(ast::Primary::Float(*val)),
            Some(Token::VARIABLE(val)) => {
                if !self.variables.contains(val) {
                    Err(anyhow!("Variable referenced before assignment!"))?
                }
                Ok(ast::Primary::Variable(val.clone()))
            }
            Some(token) => Err(anyhow!("Unexpected token! {token}"))?,
            None => Err(anyhow!("Unexpected token! None"))?,
        }
    }
}
