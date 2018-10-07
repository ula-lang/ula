use std::fmt;

use ast::Expr;
use ast::exprs::FCall;
use compilation::Compilable;

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
    fn compile(&self) -> String {
        format!("{}:{}", self.lhs.compile(), self.f_call.compile())
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