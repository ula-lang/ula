use super::super::Expr;

use std::fmt;

use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct Mod {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Mod {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Mod {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} % {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for Mod {
    fn into(self) -> Expr {
        Expr::Mod(self)
    }
}

impl fmt::Debug for Mod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mod({:?}, {:?})", self.lhs, self.rhs)
    }
}
