use std::fmt;

use ast::Expr;
use compilation::Compilable;

#[derive(Clone)]
pub struct Gt {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Gt {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Gt {
    fn compile(&self) -> String {
        format!("({} > {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for Gt {
    fn into(self) -> Expr {
        Expr::Gt(self)
    }
}

impl fmt::Debug for Gt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Gt({:?}, {:?})", self.lhs, self.rhs)
    }
}
