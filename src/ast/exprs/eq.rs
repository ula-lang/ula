use super::super::Expr;

use std::fmt;

use compilation::Compilable;

#[derive(Clone)]
pub struct Eq {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Eq {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Eq {
    fn compile(&self) -> String {
        format!("({} == {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for Eq {
    fn into(self) -> Expr {
        Expr::Eq(self)
    }
}

impl fmt::Debug for Eq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Eq({:?}, {:?})", self.lhs, self.rhs)
    }
}
