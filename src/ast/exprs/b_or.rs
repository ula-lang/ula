use super::super::Expr;

use std::fmt;

use compilation::Compilable;

#[derive(Clone)]
pub struct BOr {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl BOr {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for BOr {
    fn compile(&self) -> String {
        format!("bit.bor({}, {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for BOr {
    fn into(self) -> Expr {
        Expr::BOr(self)
    }
}

impl fmt::Debug for BOr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BOr({:?}, {:?})", self.lhs, self.rhs)
    }
}
