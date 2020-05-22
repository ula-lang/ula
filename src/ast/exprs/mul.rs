use std::{fmt, io};

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};
use ptree::{TreeItem, Style};
use std::borrow::Cow;

#[derive(Clone)]
pub struct Mul {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Mul {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Into<Expr> for Mul {
    fn into(self) -> Expr {
        Expr::Mul(self)
    }
}

impl Compilable for Mul {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} * {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl fmt::Debug for Mul {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Mul({:?}, {:?})", self.lhs, self.rhs)
    }
}
