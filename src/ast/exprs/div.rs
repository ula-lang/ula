use std::fmt;

use ast::Expr;
use compilation::Compilable;

#[derive(Clone)]
pub struct Div {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Div {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Into<Expr> for Div {
    fn into(self) -> Expr {
        Expr::Div(self)
    }
}

impl Compilable for Div {
    fn compile(&self) -> String {
        format!("({} / {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl fmt::Debug for Div {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Div({:?}, {:?})", self.lhs, self.rhs)
    }
}