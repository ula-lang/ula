use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};
use std::fmt;

#[derive(Clone)]
pub struct Dot {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

impl Dot {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into()),
        }
    }
}

impl Compilable for Dot {
    fn compile(&self, scope: &Scope) -> String {
        format!("{}.{}", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for Dot {
    fn into(self) -> Expr {
        Expr::Dot(self)
    }
}

impl fmt::Debug for Dot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dot({:?}, {:?})", self.lhs, self.rhs)
    }
}