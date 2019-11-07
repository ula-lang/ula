use super::super::Expr;

use std::fmt;

use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct Lt {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Lt {
    pub fn new(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs)
        }
    }
}

impl Compilable for Lt {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} < {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for Lt {
    fn into(self) -> Expr {
        Expr::Lt(self)
    }
}

impl fmt::Debug for Lt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lt({:?}, {:?})", self.lhs, self.rhs)
    }
}
