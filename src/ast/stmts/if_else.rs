use std::fmt;

use crate::ast::{Expr, Stmt};
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct IfElse {
    cond: Expr,
    body: Vec<Stmt>,
    else_body: Option<Vec<Stmt>>,
}

impl IfElse {
    pub fn new<T: Into<Expr>>(cond: T, body: Vec<Stmt>, else_body: Option<Vec<Stmt>>) -> Self {
        Self {
            cond: cond.into(),
            body,
            else_body,
        }
    }
}

impl Compilable for IfElse {
    fn compile(&self, scope: &Scope) -> String {
        let scope = &scope.create_child();

        let mut compiled = String::new();

        compiled.push_str(&format!("if {} then\r\n{}\r\n", self.cond.compile(scope), self.body.compile_indented(scope, 1)));

        if let Some(ref else_body) = self.else_body {
            compiled.push_str(&format!("else\r\n{}\r\n", else_body.compile_indented(scope, 1)));
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