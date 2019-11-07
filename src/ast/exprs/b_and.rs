use super::super::Expr;

use std::fmt;

use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct BAnd {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl BAnd {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for BAnd {
    fn compile(&self, scope: &Scope) -> String {
        format!("bit.band({}, {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for BAnd {
    fn into(self) -> Expr {
        Expr::BAnd(self)
    }
}

impl fmt::Debug for BAnd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BAnd({:?}, {:?})", self.lhs, self.rhs)
    }
}
