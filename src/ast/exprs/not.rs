use super::super::Expr;

use std::fmt;

use compilation::Compilable;

/// Unary not expression ("!")
#[derive(Clone)]
pub struct Not {
    rhs: Box<Expr>
}

impl Not {
    pub fn new<R>(rhs: R) -> Self where R: Into<Expr> {
        Self {
            rhs: Box::new(rhs.into())
        }
    }
}

impl Compilable for Not {
    fn compile(&self) -> String {
        format!("(not {})", self.rhs.compile())
    }
}

impl Into<Expr> for Not {
    fn into(self) -> Expr {
        Expr::Not(self)
    }
}

impl fmt::Debug for Not {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not({:?})", self.rhs)
    }
}