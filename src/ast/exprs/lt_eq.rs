use super::super::Expr;

use std::fmt;

use compilation::Compilable;

#[derive(Clone)]
pub struct LtEq {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl LtEq {
    pub fn new(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs)
        }
    }
}

impl Compilable for LtEq {
    fn compile(&self) -> String {
        format!("({} <= {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for LtEq {
    fn into(self) -> Expr {
        Expr::LtEq(self)
    }
}

impl fmt::Debug for LtEq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LtEq({:?}, {:?})", self.lhs, self.rhs)
    }
}
