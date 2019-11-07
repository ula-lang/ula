use std::fmt;

use crate::ast::Expr;
use crate::ast::exprs::FCall;
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct MCall {
    lhs: Box<Expr>,
    f_call: FCall
}

impl MCall {
    pub fn new<L>(lhs: L, f_call: FCall) -> Self where L: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            f_call
        }
    }
}

impl Compilable for MCall {
    fn compile(&self, scope: &Scope) -> String {
        format!("{}:{}", self.lhs.compile(scope), self.f_call.compile(scope))
    }
}

impl Into<Expr> for MCall {
    fn into(self) -> Expr {
        Expr::MCall(self)
    }
}

impl fmt::Debug for MCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MCall({:?}, {:?})", self.lhs, self.f_call)
    }
}