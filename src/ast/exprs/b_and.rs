use super::super::Expr;

use std::fmt;

use compilation::Compilable;

#[derive(Clone)]
pub struct BAnd {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl BAnd {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for BAnd {
    fn compile(&self) -> String {
        format!("bit.band({}, {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for BAnd {
    fn into(self) -> Expr {
        Expr::BAnd(self)
    }
}

impl fmt::Debug for BAnd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BAnd({:?}, {:?})", self.lhs, self.rhs)
    }
}
