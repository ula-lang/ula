use std::fmt;

use ast::Expr;
use compilation::Compilable;

#[derive(Clone)]
pub struct Sum {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Sum {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Into<Expr> for Sum {
    fn into(self) -> Expr {
        Expr::Sum(self)
    }
}

impl Compilable for Sum {
    fn compile(&self) -> String {
        format!("({} + {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl fmt::Debug for Sum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sum({:?}, {:?})", self.lhs, self.rhs)
    }
}