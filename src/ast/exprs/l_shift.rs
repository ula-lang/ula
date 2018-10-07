use super::super::Expr;

use std::fmt;

use compilation::Compilable;

#[derive(Clone)]
pub struct LShift {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl LShift {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for LShift {
    fn compile(&self) -> String {
        format!("(bit.lshift({}, {}))", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for LShift {
    fn into(self) -> Expr {
        Expr::LShift(self)
    }
}

impl fmt::Debug for LShift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LShift({:?}, {:?})", self.lhs, self.rhs)
    }
}
