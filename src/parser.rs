use crate::ast;
use crate::lexer::{Lexer, Token};
use anyhow::{anyhow, Ok, Result};
use std::collections::HashSet;
use std::iter::Peekable;

#[derive(Default)]
pub struct Parser {
    pub ast: ast::Ast,
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
            let statement = self.statement(&mut tokens)?;

            self.ast.program.push(statement);
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
    fn statement<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::NEWLINE) => Ok(self.statement(tokens)?),
            Some(Token::PRINT) => Ok(self.statement_print(tokens)?),
            Some(Token::IF) => Ok(self.statement_if(tokens)?),
            Some(Token::WHILE) => Ok(self.statement_while(tokens)?),
            Some(Token::LABEL) => Ok(self.statement_label(tokens)?),
            Some(Token::GOTO) => Ok(self.statement_goto(tokens)?),
            Some(Token::LET) => Ok(self.statement_let(tokens)?),
            Some(Token::INPUT) => Ok(self.statement_input(tokens)?),
            Some(token) => Err(anyhow!("Invalid statement at: {token}"))?,
            None => panic!("None encountered!!!"),
        }
    }

    fn var<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<String>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::VARIABLE(content)) => Ok(content.clone()),
            _ => Err(anyhow!("Invalid variable!"))?,
        }
    }

    fn nl<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<()>
    where
        I: Iterator<Item = &'a Token>,
    {
        match tokens.next() {
            Some(Token::NEWLINE) => Ok(()),
            val => Err(anyhow!("Should be followed by NEWLINE: {:?}", val))?,
        }
    }

    // statement ::= PRINT (expression | string) nl
    fn statement_print<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token>,
    {
        let return_value = match tokens.next() {
            Some(Token::STRING(value)) => Ok(ast::Statement::PrintStr(value.clone())),
            _ => Ok(ast::Statement::Print(self.expression(tokens)?)),
        };

        self.nl(tokens)?;

        return_value
    }

    // statement ::= IF comparison "THEN" nl {statement} "ENDIF" nl
    fn statement_if<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token>,
    {
        let comparison = self.comparison(tokens)?;

        match tokens.next() {
            Some(Token::THEN) => (),
            val => return Err(anyhow!("IF should be followed by THEN, got {:?}", val)),
        }

        self.nl(tokens)?;

        let statement = Box::new(self.statement(tokens)?);

        match tokens.next() {
            Some(Token::ENDIF) => {
                tokens.next();
                self.nl(tokens)?;
                Ok(ast::Statement::If(comparison, statement))
            }
            _ => Err(anyhow!("IF should be followed by ENDIF")),
        }
    }

    // statement ::= WHILE comparison "REPEAT" nl {statement} "ENDWHILE" nl
    fn statement_while<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token>,
    {
        let comparison = self.comparison(tokens)?;

        match tokens.next() {
            Some(Token::REPEAT) => (),
            val => return Err(anyhow!("WHILE should be followed by REPEAT, got {:?}", val)),
        }

        self.nl(tokens)?;

        let statement = Box::new(self.statement(tokens)?);

        match tokens.next() {
            Some(Token::ENDWHILE) => {
                tokens.next();
                self.nl(tokens)?;
                Ok(ast::Statement::While(comparison, statement))
            }
            val => Err(anyhow!(
                "WHILE should be followed by ENDWHILE, got: {:?}",
                val
            )),
        }
    }

    // statement ::= LABEL var nl
    fn statement_label<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token>,
    {
        let var = self.var(tokens)?;

        if !self.labels_declared.insert(var.clone()) {
            return Err(anyhow!("Label aready exists! {var}"));
        }

        self.nl(tokens)?;
        Ok(ast::Statement::Label(var))
    }

    // statement ::= GOTO var nl
    fn statement_goto<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token>,
    {
        let var = self.var(tokens)?;

        self.labels_gotoed.insert(var.clone());

        self.nl(tokens)?;

        Ok(ast::Statement::Goto(var))
    }

    // statement ::= LET var "=" expression nl
    fn statement_let<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token>,
    {
        let var = self.var(tokens)?;

        self.variables.insert(var.clone());

        let result = match tokens.next() {
            Some(Token::EQ) => self.expression(tokens)?,
            val => return Err(anyhow!("LET should be followed by '=', got {:?}", val)),
        };

        self.nl(tokens)?;

        Ok(ast::Statement::Let(var, result))
    }

    // statement ::= INPUT var nl
    fn statement_input<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Statement>
    where
        I: Iterator<Item = &'a Token>,
    {
        let var = self.var(tokens)?;

        self.variables.insert(var.clone());

        self.nl(tokens)?;

        Ok(ast::Statement::Input(var))
    }

    // comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
    fn comparison<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Comparison>
    where
        I: Iterator<Item = &'a Token>,
    {
        let left_expr = self.expression(tokens)?;

        let right_expr = match tokens.peek() {
            Some(Token::EQEQ) => {
                tokens.next();
                ast::Comparison::Compare("==".to_string(), Box::new(self.comparison(tokens)?))
            }
            Some(Token::NOTEQ) => {
                tokens.next();
                ast::Comparison::Compare("!=".to_string(), Box::new(self.comparison(tokens)?))
            }
            Some(Token::LT) => {
                tokens.next();
                ast::Comparison::Compare("<".to_string(), Box::new(self.comparison(tokens)?))
            }
            Some(Token::LTEQ) => {
                tokens.next();
                ast::Comparison::Compare("<=".to_string(), Box::new(self.comparison(tokens)?))
            }
            Some(Token::GT) => {
                tokens.next();
                ast::Comparison::Compare(">".to_string(), Box::new(self.comparison(tokens)?))
            }
            Some(Token::GTEQ) => {
                tokens.next();
                ast::Comparison::Compare(">=".to_string(), Box::new(self.comparison(tokens)?))
            }
            _ => return Ok(ast::Comparison::Right(left_expr)),
        };

        Ok(ast::Comparison::Left(left_expr, Box::new(right_expr)))
    }

    // expression ::= term {( "-" | "+" ) term}
    fn expression<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Expression>
    where
        I: Iterator<Item = &'a Token>,
    {
        let term = self.term(tokens)?;

        match tokens.peek() {
            Some(Token::PLUS) => {
                tokens.next();
                Ok(ast::Expression::Add(Box::new(self.expression(tokens)?)))
            }
            Some(Token::MINUS) => {
                tokens.next();
                Ok(ast::Expression::Add(Box::new(self.expression(tokens)?)))
            }
            _ => Ok(ast::Expression::Term(term)),
        }
    }

    // term ::= unary {( "/" | "*" ) unary}
    fn term<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Term>
    where
        I: Iterator<Item = &'a Token>,
    {
        let unary = self.unary(tokens)?;

        match tokens.peek() {
            Some(Token::ASTERISK) => {
                tokens.next();
                Ok(ast::Term::Mul(Box::new(self.term(tokens)?)))
            }
            Some(Token::SLASH) => {
                tokens.next();
                Ok(ast::Term::Div(Box::new(self.term(tokens)?)))
            }
            _ => Ok(ast::Term::Unary(unary)),
        }
    }

    // unary ::= ["+" | "-"] primary
    fn unary<'a, I>(&mut self, tokens: &mut Peekable<I>) -> Result<ast::Unary>
    where
        I: Iterator<Item = &'a Token>,
    {
        // optional unary
        match tokens.peek() {
            Some(Token::PLUS) => {
                tokens.next();
                Ok(ast::Unary::Positive(Box::new(self.unary(tokens)?)))
            }
            Some(Token::MINUS) => {
                tokens.next();
                Ok(ast::Unary::Negative(Box::new(self.unary(tokens)?)))
            }
            _ => Ok(ast::Unary::Primary(self.primary(tokens)?)),
        }
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
