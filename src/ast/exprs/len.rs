use super::super::Expr;

use std::fmt;

use compilation::Compilable;

/// Unary length expression (#<expr>)
#[derive(Clone)]
pub struct Len {
    rhs: Box<Expr>
}

impl Len {
    pub fn new<R>(rhs: R) -> Self where R: Into<Expr> {
        Self {
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Len {
    fn compile(&self) -> String {
        format!("(#{})", self.rhs.compile())
    }
}

impl Into<Expr> for Len {
    fn into(self) -> Expr {
        Expr::Len(self)
    }
}

impl fmt::Debug for Len {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Length({:?})", self.rhs)
    }
}