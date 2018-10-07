use std::fmt;

use ast::Expr;
use compilation::Compilable;

#[derive(Clone)]
pub struct Parens {
    inner: Box<Expr>
}

impl Parens {
    pub fn new<E>(expr: E) -> Self where E: Into<Expr> {
        Self {
            inner: Box::new(expr.into())
        }
    }
}

impl Compilable for Parens {
    fn compile(&self) -> String {
        format!("({})", self.inner.compile())
    }
}

impl From<Expr> for Parens {
    fn from(expr: Expr) -> Self {
        Parens::new(expr)
    }
}

impl Into<Expr> for Parens {
    fn into(self) -> Expr {
        Expr::Bracketed(self)
    }
}

impl fmt::Debug for Parens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bracketed({:?})", self.inner)
    }
}