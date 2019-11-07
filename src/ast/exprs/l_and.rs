use super::super::Expr;

use std::fmt;

use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct LAnd {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl LAnd {
    pub fn new(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs)
        }
    }
}

impl Compilable for LAnd {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} and {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for LAnd {
    fn into(self) -> Expr {
        Expr::LAnd(self)
    }
}

impl fmt::Debug for LAnd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LAnd({:?}, {:?})", self.lhs, self.rhs)
    }
}
