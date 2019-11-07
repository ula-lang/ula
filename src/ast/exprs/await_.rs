use super::super::Expr;

use std::fmt;

use crate::compilation::{Compilable, Scope};

/// Await expression (await <expr>)
#[derive(Clone)]
pub struct Await {
    expr: Box<Expr>
}

impl Await {
    pub fn new<E>(expr: E) -> Self where E: Into<Expr> {
        Self {
            expr: Box::new(expr.into())
        }
    }
}

impl Compilable for Await {
    fn compile(&self, scope: &Scope) -> String {
        format!("{}:GetAwaiter():GetResult()", self.expr.compile(scope))
    }
}

impl Into<Expr> for Await {
    fn into(self) -> Expr {
        Expr::Await(self)
    }
}

impl fmt::Debug for Await {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Await({:?})", self.expr)
    }
}