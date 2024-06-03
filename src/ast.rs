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

#[derive(Debug)]
pub enum Statement {
    Print(Expression),
    PrintStr(String),
    Let(String, Expression),
    If(Comparison, Box<Statement>),
    While(Comparison, Box<Statement>),
    Label(String),
    Goto(String),
    Input(String),
    Statement(Box<Statement>, Box<Statement>),
    End,
}

#[derive(Debug)]
pub enum Comparison {
    Right(Expression),
    Left(Expression, Box<Comparison>),
    Compare(String, Box<Comparison>),
}

#[derive(Debug)]
pub enum Expression {
    Term(Term),
    Add(Box<Term>, Box<Expression>),
    Neg(Box<Term>, Box<Expression>),
}

#[derive(Debug)]
pub enum Term {
    Unary(Unary),
    Mul(Box<Unary>, Box<Term>),
    Div(Box<Unary>, Box<Term>),
}

#[derive(Debug)]
pub enum Unary {
    Primary(Primary),
    Positive(Box<Unary>),
    Negative(Box<Unary>),
}

#[derive(Debug)]
pub enum Primary {
    Integer(i64),
    Float(f64),
    Number(usize),
    Variable(String),
}
