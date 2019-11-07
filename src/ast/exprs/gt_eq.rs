use std::fmt;

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct GtEq {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl GtEq {
    pub fn new(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs)
        }
    }
}

impl Compilable for GtEq {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} >= {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for GtEq {
    fn into(self) -> Expr {
        Expr::GtEq(self)
    }
}

impl fmt::Debug for GtEq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GtEq({:?}, {:?})", self.lhs, self.rhs)
    }
}
