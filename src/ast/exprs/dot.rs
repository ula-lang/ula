use ast::Expr;
use compilation::Compilable;
use std::fmt;

#[derive(Clone)]
pub struct Dot {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

impl Dot {
    pub fn new<L, R>(lhs: L, rhs: R) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into()),
        }
    }
}

impl Compilable for Dot {
    fn compile(&self) -> String {
        format!("{}.{}", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for Dot {
    fn into(self) -> Expr {
        Expr::Dot(self)
    }
}

impl fmt::Debug for Dot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Dot({:?}, {:?})", self.lhs, self.rhs)
    }
}