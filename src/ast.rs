// Syntax Overview

// {} - 0 or more
// [] - 0 or 1, i.e. optional
// () - grouping / or
// +  - 1 or more

// program ::= {statement}

// statement ::= PRINT (expression | string) nl
//               IF comparison "THEN" nl {statement} "ENDIF" nl
//               WHILE comparison "REPEAT" nl {statement} "ENDWHILE" nl
//               LABEL var nl
//               GOTO var nl
//               LET var "=" expression nl
//               INPUT var nl

// comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
// expression ::= term {( "-" | "+" ) term}
// term ::= unary {( "/" | "*" ) unary}
// unary ::= ["+" | "-"] primary
// primary ::= number | var

// term -> +-primary {*/ +-primary}

use crate::lexer::Token;

#[derive(Default)]
pub struct Ast {
    pub program: Vec<Statement>,
    line: Option<Vec<Token>>,
    tokens: Vec<Vec<Token>>,
}

pub enum Statement {
    Print(Print),
    Let(Var, Expression),
    If(Comparison, Box<Statement>),
    While(Comparison, Box<Statement>),
    Label(Var),
    Goto(Var),
    Input(Var),
}

pub enum Print {
    Expression,
    String,
}

pub enum Expression {
    Primary,
    BinaryOp,
}

pub enum Comparison {
    Expression,
    CompareOp(Box<Comparison>),
}

pub enum CompareOp {
    String,
}

pub enum BinaryOp {
    Add(Box<Expression>),
    Sub(Box<Expression>),
    Mul(Box<Expression>),
    Div(Box<Expression>),
}

pub enum Primary {
    Integer(i64),
    Float(f64),
    Var,
}

pub enum Var {
    String,
}
