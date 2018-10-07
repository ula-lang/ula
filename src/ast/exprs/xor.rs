use super::super::Expr;

use std::fmt;

use compilation::Compilable;

#[derive(Clone)]
pub struct Xor {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Xor {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Xor {
    fn compile(&self) -> String {
        format!("bit.bxor({}, {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for Xor {
    fn into(self) -> Expr {
        Expr::Xor(self)
    }
}

impl fmt::Debug for Xor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Xor({:?}, {:?})", self.lhs, self.rhs)
    }
}
