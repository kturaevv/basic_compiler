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

#[derive(Default)]
pub struct Ast {
    pub program: Vec<Statement>,
}

pub enum Statement {
    Print(Print),
    Let(String, Expression),
    If(Comparison, Box<Statement>),
    While(Comparison, Box<Statement>),
    Label(String),
    Goto(String),
    Input(String),
}

pub enum Print {
    Expression,
    String,
}

pub enum Comparison {
    Expression,
    CompareOp(CompareOp, Box<Comparison>),
}

pub enum CompareOp {
    String,
}

pub enum Expression {
    Term(Term),
    Add(Box<Expression>),
    Neg(Box<Expression>),
}

pub enum Term {
    Unary(Unary),
    Mul(Box<Term>),
    Div(Box<Term>),
}

pub enum Unary {
    Primary(Primary),
    Positive(Box<Unary>),
    Negative(Box<Unary>),
}

pub enum Primary {
    Integer(i64),
    Float(f64),
    Number(usize),
    Variable(String),
}
