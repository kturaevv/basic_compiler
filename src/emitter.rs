use std::collections::HashSet;

use crate::{
    ast::{Comparison, Expression, Primary, Statement, Term, Unary},
    parser::Parser,
};

#[derive(Default)]
pub struct Emitter {
    full_path: String,
    header: String,
    code: String,
    variables: Option<HashSet<String>>,
}

impl Emitter {
    pub fn new(full_path: &str) -> Emitter {
        Emitter {
            full_path: full_path.to_string(),
            ..Default::default()
        }
    }

    pub fn process(&mut self, parser: Parser) {
        self.variables = Some(parser.variables);

        self.emit_header("#include <stdio.h>");
        self.emit_header("int main(void){");

        for stmnt in parser.ast.program {
            let code = self.gen_statement(&stmnt);
            self.emit(code.as_str());
        }

        self.emit_line("return 0;");
        self.emit_line("}");
    }

    pub fn write_to_file(&self) {
        let content = [self.header.as_bytes(), self.code.as_bytes()].concat();
        std::fs::write(self.full_path.as_str(), content).expect("Couldnt write to file");
    }

    fn emit(&mut self, code: &str) {
        self.code.push_str(code);
    }

    fn emit_line(&mut self, code: &str) {
        self.emit(code);
        self.code.push('\n');
    }

    fn emit_header(&mut self, header: &str) {
        self.header.push_str(header);
        self.header.push('\n');
    }

    fn gen_statement(&mut self, s: &Statement) -> String {
        match s {
            Statement::Print(expr) => {
                format!("printf(\"%d\\n\", {});\n", self.gen_expression(expr))
            }
            Statement::PrintStr(string) => {
                format!("printf(\"{}\\n\");\n", string)
            }
            Statement::Let(var, expr) => {
                format!(
                    "{} = {};\n",
                    self.gen_primary(&Primary::Variable(var.to_string())),
                    self.gen_expression(expr)
                )
            }
            Statement::If(comp, stmt) => {
                format!(
                    "if ({}) {{\n{}\n}}\n",
                    self.gen_comparison(comp),
                    self.gen_statement(stmt)
                )
            }
            Statement::While(comp, stmt) => {
                format!(
                    "while ({}) {{\n{}\n}}\n",
                    self.gen_comparison(comp),
                    self.gen_statement(stmt)
                )
            }
            Statement::Label(label) => {
                format!("{}:\n", label)
            }
            Statement::Goto(label) => {
                format!("goto {};\n", label)
            }
            Statement::Input(var) => {
                // Check if var is a string or an integer

                let typed_var = self.gen_primary(&Primary::Variable(var.to_string()));
                format!("{typed_var};\nscanf(\"%d\", &{});\n", var)
            }
            Statement::Statement(stmt1, stmt2) => {
                format!(
                    "{}\n{}",
                    self.gen_statement(stmt1),
                    self.gen_statement(stmt2)
                )
            }
            Statement::End => "\n".to_string(),
        }
    }

    fn gen_comparison(&mut self, c: &Comparison) -> String {
        match c {
            Comparison::Left(expr) => self.gen_expression(expr),
            Comparison::Right(expr, comp) => {
                format!(
                    "{} {}",
                    self.gen_expression(expr),
                    self.gen_comparison(comp)
                )
            }
            Comparison::Compare(op, comp) => {
                format!("{} {}", op, self.gen_comparison(comp))
            }
        }
    }

    fn gen_expression(&mut self, e: &Expression) -> String {
        match e {
            Expression::Term(term) => self.gen_term(term),
            Expression::Add(term, expr) => {
                format!("{} + {}", self.gen_term(term), self.gen_expression(expr))
            }
            Expression::Sub(term, expr) => {
                format!("{} - {}", self.gen_term(term), self.gen_expression(expr))
            }
        }
    }

    fn gen_term(&mut self, t: &Term) -> String {
        match t {
            Term::Unary(u) => self.gen_unary(u).to_string(),
            Term::Mul(left, right) => {
                format!("{} * {}", self.gen_unary(left), self.gen_term(right))
            }
            Term::Div(left, right) => {
                format!("{} / {}", self.gen_unary(left), self.gen_term(right))
            }
        }
    }
    fn gen_unary(&mut self, u: &Unary) -> String {
        match u {
            Unary::Primary(p) => self.gen_primary(p).to_string(),
            Unary::Positive(u) => format!("+{}", self.gen_unary(u)),
            Unary::Negative(u) => format!("-{}", self.gen_unary(u)),
        }
    }
    fn gen_primary(&mut self, p: &Primary) -> String {
        match p {
            Primary::Integer(v) => format!("{}", v),
            Primary::Float(v) => format!("{}", v),
            Primary::Number(v) => format!("{}", v),
            Primary::Variable(v) => {
                let decl = match self.variables.as_ref().unwrap().contains(v) {
                    true => {
                        self.variables.as_mut().unwrap().remove(v);
                        "int"
                    }
                    false => "",
                };
                format!("{decl} {}", v)
            }
        }
    }
}
