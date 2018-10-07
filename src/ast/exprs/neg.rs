use super::super::Expr;

use std::fmt;

use compilation::Compilable;

/// Unary negation expression (-<expr>)
#[derive(Clone)]
pub struct Neg {
    rhs: Box<Expr>
}

impl Neg {
    pub fn new<R>(rhs: R) -> Self where R: Into<Expr> {
        Self {
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Neg {
    fn compile(&self) -> String {
        format!("(-{})", self.rhs.compile())
    }
}

impl Into<Expr> for Neg {
    fn into(self) -> Expr {
        Expr::Neg(self)
    }
}

impl fmt::Debug for Neg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Neg({:?})", self.rhs)
    }
}