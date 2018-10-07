use super::super::Expr;

use std::fmt;

use compilation::Compilable;

#[derive(Clone)]
pub struct Ne {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Ne {
    pub fn new(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs)
        }
    }
}

impl Compilable for Ne {
    fn compile(&self) -> String {
        format!("({} ~= {})", self.lhs.compile(), self.rhs.compile())
    }
}

impl Into<Expr> for Ne {
    fn into(self) -> Expr {
        Expr::Ne(self)
    }
}

impl fmt::Debug for Ne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ne({:?}, {:?})", self.lhs, self.rhs)
    }
}
