use std::fmt;

use ast::Expr;
use compilation::Compilable;

#[derive(Clone)]
pub struct Assignment {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Assignment {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Assignment {
    fn compile(&self) -> String {
        format!("{} = {}", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for Assignment {
    fn into(self) -> Expr {
        Expr::Assignment(self)
    }
}

impl fmt::Debug for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assignment({:?}, {:?})", self.lhs, self.rhs)
    }
}