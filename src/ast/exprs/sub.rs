use std::fmt;

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct Sub {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Sub {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Into<Expr> for Sub {
    fn into(self) -> Expr {
        Expr::Sub(self)
    }
}

impl Compilable for Sub {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} - {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl fmt::Debug for Sub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sub({:?}, {:?})", self.lhs, self.rhs)
    }
}