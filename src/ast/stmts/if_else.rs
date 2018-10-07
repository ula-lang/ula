use std::fmt;

use ast::{Expr, Stmt};
use compilation::Compilable;

#[derive(Clone)]
pub struct IfElse {
    cond: Expr,
    body: Vec<Stmt>,
    else_body: Option<Vec<Stmt>>
}

impl IfElse {
    pub fn new<T: Into<Expr>>(cond: T, body: Vec<Stmt>, else_body: Option<Vec<Stmt>>) -> Self {
        Self {
            cond: cond.into(),
            body,
            else_body
        }
    }
}

impl Compilable for IfElse {
    fn compile(&self) -> String {
        let mut compiled = String::new();

        compiled.push_str(&format!("if {} then\r\n{}\r\n", self.cond.compile(), self.body.compile_indented(1)));

        if let Some(ref else_body) = self.else_body {
            compiled.push_str(&format!("else\r\n{}\r\n", else_body.compile_indented(1)));
        }

        compiled.push_str("end");

        compiled
    }
}

impl Into<Stmt> for IfElse {
    fn into(self) -> Stmt {
        Stmt::IfElse(self)
    }
}

impl fmt::Debug for IfElse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "If(Cond({:?}), Body({:?})", self.cond, self.body)?;

        if let Some(ref else_body) = self.else_body {
            write!(f, ", ElseBody({:?})", else_body)?;
        }

        write!(f, ")")
    }
}