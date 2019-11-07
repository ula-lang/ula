use std::fmt;

use crate::ast::Expr;
use crate::ast::exprs::{Const, FCall};
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct New {
    ident: String,
    ctor_call: FCall
}

impl New {
    pub fn new(ident: String, mut ctor_call: FCall) -> Self {
        ctor_call.args_mut().insert(0, Const::Nil.into());

        Self {
            ident,
            ctor_call
        }
    }
}

impl Compilable for New {
    fn compile(&self, scope: &Scope) -> String {
        format!("{}.{}", self.ident, self.ctor_call.compile(scope))
    }
}

impl Into<Expr> for New {
    fn into(self) -> Expr {
        Expr::New(self)
    }
}

impl fmt::Debug for New {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "New({}, {:?})", self.ident, self.ctor_call)
    }
}