use std::fmt;

use ast::Expr;
use compilation::Compilable;

#[derive(Clone)]
pub struct Sub {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Sub {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Into<Expr> for Sub {
    fn into(self) -> Expr {
        Expr::Sub(self)
    }
}

impl Compilable for Sub {
    fn compile(&self) -> String {
        format!("({} - {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl fmt::Debug for Sub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sub({:?}, {:?})", self.lhs, self.rhs)
    }
}