use std::fmt;

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct Assignment {
    lhs: Vec<Expr>,
    rhs: Vec<Expr>
}

impl Assignment {
    pub fn new<L, R>(lhs: Vec<L>, rhs: Vec<R>) -> Self where L: Into<Expr>, R: Into<Expr> {
        Self {
            lhs: lhs.into_iter().map(|value| value.into()).collect(),
            rhs: rhs.into_iter().map(|value| value.into()).collect()
        }
    }
}

impl Compilable for Assignment {
    fn compile(&self, scope: &Scope) -> String {
        self.lhs.iter().for_each(|expr| {
            if let Expr::Ref(ref_) = expr {
                scope.add_variable(ref_.ident())
            }
        });

        format!("{} = {}", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl Into<Expr> for Assignment {
    fn into(self) -> Expr {
        Expr::Assignment(self)
    }
}

impl fmt::Debug for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Assignment({:?}, {:?})", self.lhs, self.rhs)
    }
}
