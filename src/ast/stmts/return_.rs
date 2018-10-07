use std::fmt;

use ast::{Expr, Stmt};
use compilation::Compilable;

#[derive(Clone)]
pub struct Return {
    expr: Option<Expr>
}

impl Return {
    pub fn new<E>(expr: Option<E>) -> Self where E: Into<Expr> {
        Self {
            expr: expr.map(Into::into)
        }
    }
}

impl Compilable for Return {
    fn compile(&self) -> String {
        let mut compiled = String::new();

        compiled.push_str("return");

        if let Some(ref expr) = self.expr {
            compiled.push_str(&format!(" {}", expr.compile()));
        }

        compiled.push(';');

        compiled
    }
}

impl Into<Stmt> for Return {
    fn into(self) -> Stmt {
        Stmt::Return(self)
   }
}

impl fmt::Debug for Return {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Return({:?})", self.expr)
    }
}
