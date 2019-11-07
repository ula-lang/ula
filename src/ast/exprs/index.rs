use std::fmt;

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct Index {
    lhs: Box<Expr>,
    index: Box<Expr>
}

impl Index {
    pub fn new<L, I>(lhs: L, index: I) -> Self where L: Into<Expr>, I: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            index: Box::new(index.into())
        }
    }
}

impl Compilable for Index {
    fn compile(&self, scope: &Scope) -> String {
        format!("{}[{}]", self.lhs.compile(scope), self.index.compile(scope))
    }
}

impl Into<Expr> for Index {
    fn into(self) -> Expr {
        Expr::Index(self)
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index({:?}, {:?})", self.lhs, self.index)
    }
}
