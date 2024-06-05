use crate::ast;
use crate::lexer::{Lexer, Token};

use anyhow::{anyhow, Ok, Result};
use tracing::{self, instrument};

use std::collections::HashSet;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Default)]
pub struct Parser {
    pub ast: ast::Ast,
    pub variables: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_gotoed: HashSet<String>,
    iter: Option<Peekable<IntoIter<Token>>>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            iter: None,
            ..Default::default()
        }
    }

    fn peek(&mut self) -> Option<Token> {
        self.iter.as_mut().unwrap().peek().cloned()
    }

    fn advance(&mut self) -> Option<Token> {
        self.iter.as_mut().unwrap().next()
    }

    /// program ::= {statement}
    pub fn check(&mut self, lexer: &Lexer) -> Result<()> {
        self.iter = Some(lexer.tokens.clone().into_iter().peekable());

        while self.peek().is_some() {
            let statement = self.statement()?;

            self.ast.program.push(statement);
        }

        for label in &self.labels_gotoed {
            if self.labels_declared.contains(label) {
                return Err(anyhow!("Attemt to GOTO to undeclared label! {label}"));
            }
        }

        tracing::debug!("{:#?}", self.ast);

        Ok(())
    }

    /// statement ::= PRINT (expression | string) nl
    ///               IF comparison "THEN" nl {statement} "ENDIF" nl
    ///               WHILE comparison "REPEAT" nl {statement} "ENDWHILE" nl
    ///               LABEL var nl
    ///               GOTO var nl
    ///               LET var "=" expression nl
    ///               INPUT var nl
    #[tracing::instrument(skip_all)]
    fn statement(&mut self) -> Result<ast::Statement> {
        tracing::debug!("Current token {:?}", self.peek());
        tracing::debug!("{:#?}", self.ast);

        match self.peek() {
            Some(Token::NEWLINE) => {
                self.advance();
                Ok(self.statement()?)
            }
            Some(Token::PRINT) => {
                self.advance();
                Ok(self.statement_print()?)
            }
            Some(Token::IF) => {
                self.advance();
                Ok(self.statement_if()?)
            }
            Some(Token::WHILE) => {
                self.advance();
                Ok(self.statement_while()?)
            }
            Some(Token::LABEL) => {
                self.advance();
                Ok(self.statement_label()?)
            }
            Some(Token::GOTO) => {
                self.advance();
                Ok(self.statement_goto()?)
            }
            Some(Token::LET) => {
                self.advance();
                Ok(self.statement_let()?)
            }
            Some(Token::INPUT) => {
                self.advance();
                Ok(self.statement_input()?)
            }
            Some(token) => Err(anyhow!("Invalid statement at: {token}"))?,
            None => panic!("None encountered!!!"),
        }
    }

    #[tracing::instrument(skip_all)]
    fn var(&mut self) -> Result<String> {
        tracing::debug!("Current token {:?}", self.peek());

        match self.advance() {
            Some(Token::VARIABLE(content)) => Ok(content.clone()),
            _ => Err(anyhow!("Invalid variable!"))?,
        }
    }

    #[tracing::instrument(skip_all)]
    fn nl(&mut self) -> Result<()> {
        tracing::debug!("Current token {:?}", self.peek());

        match self.advance() {
            Some(Token::NEWLINE) => Ok(()),
            val => Err(anyhow!("Expected NEWLINE, got {:?}", val))?,
        }
    }

    /// statement ::= PRINT (expression | string) nl
    #[tracing::instrument(skip_all)]
    fn statement_print(&mut self) -> Result<ast::Statement> {
        tracing::debug!("Current token {:?}", self.peek());

        let return_value = match self.peek() {
            Some(Token::STRING(value)) => {
                self.advance();
                Ok(ast::Statement::PrintStr(value.clone()))
            }
            _ => Ok(ast::Statement::Print(self.expression()?)),
        };

        self.nl()?;

        return_value
    }

    /// statement ::= IF comparison "THEN" nl {statement} "ENDIF" nl
    #[tracing::instrument(skip_all)]
    fn statement_if(&mut self) -> Result<ast::Statement> {
        tracing::debug!("Current token {:?}", self.peek());
        let comparison = self.comparison()?;

        match self.advance() {
            Some(Token::THEN) => (),
            val => return Err(anyhow!("IF should be followed by THEN, got {:?}", val)),
        }

        self.nl()?;

        let statement = Box::new(self.statement()?);

        match self.advance() {
            Some(Token::ENDIF) => {
                self.nl()?;
                Ok(ast::Statement::If(comparison, statement))
            }
            _ => Err(anyhow!("IF should be followed by ENDIF")),
        }
    }

    /// statement ::= WHILE comparison "REPEAT" nl {statement} "ENDWHILE" nl
    #[tracing::instrument(skip_all)]
    fn statement_while(&mut self) -> Result<ast::Statement> {
        tracing::debug!("Current token {:?}", self.peek());

        let comparison = self.comparison()?;

        match self.advance() {
            Some(Token::REPEAT) => (),
            val => return Err(anyhow!("WHILE should be followed by REPEAT, got {:?}", val)),
        }

        self.nl()?;

        let statement = Box::new(self.chain_statements()?);

        match self.advance() {
            Some(Token::ENDWHILE) => {
                self.nl()?;
                Ok(ast::Statement::While(comparison, statement))
            }
            val => Err(anyhow!(
                "WHILE should be followed by ENDWHILE, got: {:?}",
                val
            )),
        }
    }

    #[tracing::instrument(skip_all)]
    fn chain_statements(&mut self) -> Result<ast::Statement> {
        match self.statement() {
            Result::Ok(statement) => Ok(ast::Statement::Statement(
                Box::new(statement),
                Box::new(self.chain_statements()?),
            )),
            Err(err) => {
                tracing::debug!("Chain end.");
                match self.peek().unwrap() {
                    Token::ENDWHILE => Ok(ast::Statement::End),
                    _ => Err(err),
                }
            }
        }
    }

    /// statement ::= LABEL var nl
    #[tracing::instrument(skip_all)]
    fn statement_label(&mut self) -> Result<ast::Statement> {
        tracing::debug!("Current token {:?}", self.peek());

        let var = self.var()?;

        if !self.labels_declared.insert(var.clone()) {
            return Err(anyhow!("Label aready exists! {var}"));
        }

        self.nl()?;
        Ok(ast::Statement::Label(var))
    }

    /// statement ::= GOTO var nl
    #[tracing::instrument(skip_all)]
    fn statement_goto(&mut self) -> Result<ast::Statement> {
        tracing::debug!("Current token {:?}", self.peek());

        let var = self.var()?;

        self.labels_gotoed.insert(var.clone());

        self.nl()?;

        Ok(ast::Statement::Goto(var))
    }

    /// statement ::= LET var "=" expression nl
    #[tracing::instrument(skip_all)]
    fn statement_let(&mut self) -> Result<ast::Statement> {
        tracing::debug!("Current token {:?}", self.peek());

        let var = self.var()?;

        self.variables.insert(var.clone());

        let result = match self.advance() {
            Some(Token::EQ) => self.expression()?,
            val => return Err(anyhow!("LET should be followed by '=', got {:?}", val)),
        };

        self.nl()?;

        Ok(ast::Statement::Let(var, result))
    }

    /// statement ::= INPUT var nl
    #[instrument(skip_all)]
    fn statement_input(&mut self) -> Result<ast::Statement> {
        tracing::debug!("Current token {:?}", self.peek());

        let var = self.var()?;

        self.variables.insert(var.clone());

        self.nl()?;

        Ok(ast::Statement::Input(var))
    }

    /// comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
    #[instrument(skip_all)]
    fn comparison(&mut self) -> Result<ast::Comparison> {
        tracing::debug!("Current token {:?}", self.peek());

        let left_expr = self.expression()?;

        let right_expr = match self.peek() {
            Some(Token::EQEQ) => {
                self.advance();
                ast::Comparison::Compare("==".to_string(), Box::new(self.comparison()?))
            }
            Some(Token::NOTEQ) => {
                self.advance();
                ast::Comparison::Compare("!=".to_string(), Box::new(self.comparison()?))
            }
            Some(Token::LT) => {
                self.advance();
                ast::Comparison::Compare("<".to_string(), Box::new(self.comparison()?))
            }
            Some(Token::LTEQ) => {
                self.advance();
                ast::Comparison::Compare("<=".to_string(), Box::new(self.comparison()?))
            }
            Some(Token::GT) => {
                self.advance();
                ast::Comparison::Compare(">".to_string(), Box::new(self.comparison()?))
            }
            Some(Token::GTEQ) => {
                self.advance();
                ast::Comparison::Compare(">=".to_string(), Box::new(self.comparison()?))
            }
            _ => return Ok(ast::Comparison::Left(left_expr)),
        };

        Ok(ast::Comparison::Right(left_expr, Box::new(right_expr)))
    }

    /// expression ::= term {( "-" | "+" ) term}
    #[instrument(skip_all)]
    fn expression(&mut self) -> Result<ast::Expression> {
        tracing::debug!("Current token {:?}", self.peek());

        let term = self.term()?;

        match self.peek() {
            Some(Token::PLUS) => {
                self.advance();
                Ok(ast::Expression::Add(
                    Box::new(term),
                    Box::new(self.expression()?),
                ))
            }
            Some(Token::MINUS) => {
                self.advance();
                Ok(ast::Expression::Sub(
                    Box::new(term),
                    Box::new(self.expression()?),
                ))
            }
            _ => Ok(ast::Expression::Term(term)),
        }
    }

    /// term ::= unary {( "/" | "*" ) unary}
    #[instrument(skip_all)]
    fn term(&mut self) -> Result<ast::Term> {
        tracing::debug!("Current token {:?}", self.peek());

        let unary = self.unary()?;

        match self.peek() {
            Some(Token::ASTERISK) => {
                self.advance();
                Ok(ast::Term::Mul(Box::new(unary), Box::new(self.term()?)))
            }
            Some(Token::SLASH) => {
                self.advance();
                Ok(ast::Term::Div(Box::new(unary), Box::new(self.term()?)))
            }
            _ => Ok(ast::Term::Unary(unary)),
        }
    }

    /// unary ::= ["+" | "-"] primary
    #[instrument(skip_all)]
    fn unary(&mut self) -> Result<ast::Unary> {
        tracing::debug!("Current token {:?}", self.peek());

        match self.peek() {
            Some(Token::PLUS) => {
                self.advance();
                Ok(ast::Unary::Positive(Box::new(self.unary()?)))
            }
            Some(Token::MINUS) => {
                self.advance();
                Ok(ast::Unary::Negative(Box::new(self.unary()?)))
            }
            _ => Ok(ast::Unary::Primary(self.primary()?)),
        }
    }

    /// primary ::= number | var
    #[instrument(skip_all)]
    fn primary(&mut self) -> Result<ast::Primary> {
        tracing::debug!("Current token {:?}", self.peek());

        match self.advance() {
            Some(Token::INTEGER(val)) => Ok(ast::Primary::Integer(val)),
            Some(Token::FLOAT(val)) => Ok(ast::Primary::Float(val)),
            Some(Token::VARIABLE(val)) => {
                if !self.variables.contains(val.as_str()) {
                    Err(anyhow!("Variable referenced before assignment!"))?
                }
                Ok(ast::Primary::Variable(val.clone()))
            }
            token => Err(anyhow!(
                "Unexpected token! Expecting VARIABLE, got {:?}",
                token.unwrap()
            ))?,
        }
    }
}
