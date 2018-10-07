use std::fmt;

use ast::{Expr, Stmt};
use compilation::Compilable;

// Yield statement (yield <expr?>)
#[derive(Clone)]
pub struct Yield {
    expr: Option<Expr>
}

impl Yield {
    pub fn new<E>(expr: Option<E>) -> Self where E: Into<Expr> {
        Self {
            expr: expr.map(Into::into)
        }
    }
}

impl Compilable for Yield {
    fn compile(&self) -> String {
        format!("coroutine.yield({});", self.expr.compile())
    }
}

impl Into<Stmt> for Yield {
    fn into(self) -> Stmt {
        Stmt::Yield(self)
    }
}

impl fmt::Debug for Yield {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Yield({:?})", self.expr)
    }
}
