use super::super::Expr;

use std::fmt;

use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct RShift {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl RShift {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for RShift {
    fn compile(&self, scope: &Scope) -> String {
        format!("bit.rshift({}, {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for RShift {
    fn into(self) -> Expr {
        Expr::RShift(self)
    }
}

impl fmt::Debug for RShift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RShift({:?}, {:?})", self.lhs, self.rhs)
    }
}
