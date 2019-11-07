use std::fmt;

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct Gt {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Gt {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Gt {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} > {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for Gt {
    fn into(self) -> Expr {
        Expr::Gt(self)
    }
}

impl fmt::Debug for Gt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gt({:?}, {:?})", self.lhs, self.rhs)
    }
}
