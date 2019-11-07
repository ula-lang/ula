use super::super::Expr;

use std::fmt;

use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct BOr {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl BOr {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for BOr {
    fn compile(&self, scope: &Scope) -> String {
        format!("bit.bor({}, {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for BOr {
    fn into(self) -> Expr {
        Expr::BOr(self)
    }
}

impl fmt::Debug for BOr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BOr({:?}, {:?})", self.lhs, self.rhs)
    }
}
