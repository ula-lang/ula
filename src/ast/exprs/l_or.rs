use super::super::Expr;

use std::fmt;

use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct LOr {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl LOr {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for LOr {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} or {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for LOr {
    fn into(self) -> Expr {
        Expr::LOr(self)
    }
}

impl fmt::Debug for LOr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LOr({:?}, {:?})", self.lhs, self.rhs)
    }
}
