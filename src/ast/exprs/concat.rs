use super::super::Expr;

use std::fmt;

use compilation::Compilable;

/// Concat expression (<expr> .. <expr>)
#[derive(Clone)]
pub struct Concat {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Concat {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Concat {
    fn compile(&self) -> String {
        format!("{} .. {}", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for Concat {
    fn into(self) -> Expr {
        Expr::Concat(self)
    }
}

impl fmt::Debug for Concat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Concat({:?}, {:?})", self.lhs, self.rhs)
    }
}