use std::fmt;

use ast::Expr;
use compilation::Compilable;

#[derive(Clone)]
pub struct FCall {
    lhs: Box<Expr>,
    args: Vec<Expr>
}

impl FCall {
    pub fn new<L>(lhs: L, args: Vec<Expr>) -> Self where L: Into<Expr> {
        Self {
            lhs: Box::new(lhs.into()),
            args
        }
    }

    pub fn args_mut(&mut self) -> &mut Vec<Expr> {
        &mut self.args
    }
}

impl Compilable for FCall {
    fn compile(&self) -> String {
        format!("{}({})", self.lhs.compile(), self.args.compile())
    }
}

impl Into<Expr> for FCall {
    fn into(self) -> Expr {
        Expr::FCall(self)
    }
}

impl fmt::Debug for FCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FCall({:?}", self.lhs)?;

        if self.args.len() > 0 {
            write!(f, ", Params({:?})", self.args)?;
        }

        write!(f, ")")
    }
}