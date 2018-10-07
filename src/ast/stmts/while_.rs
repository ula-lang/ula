use std::fmt;

use ast::{Expr, Stmt};
use compilation::Compilable;

#[derive(Clone)]
pub struct While {
    cond: Expr,
    body: Vec<Stmt>
}

impl While {
    pub fn new<C>(cond: C, body: Vec<Stmt>) -> Self where C: Into<Expr> {
        Self {
            cond: cond.into(),
            body
        }
    }
}

impl Compilable for While {
    fn compile(&self) -> String {
        format!("while {} do\r\n{}\r\nend", self.cond.compile(), self.body.compile_indented(1))
    }
}

impl Into<Stmt> for While {
    fn into(self) -> Stmt {
        Stmt::While(self)
    }
}

impl fmt::Debug for While {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "While(Cond({:?}), Body({:?}))", self.cond, self.body)
    }
}