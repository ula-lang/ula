use super::super::Expr;

use std::fmt;

use compilation::Compilable;

/// Conditional expression (<expr> ? <expr> : <expr>)
#[derive(Clone)]
pub struct Cond {
    cond: Box<Expr>,
    expr_a: Box<Expr>,
    expr_b: Box<Expr>,
}

impl Cond {
    pub fn new<C, EA, EB>(cond: C, expr_a: EA, expr_b: EB) -> Self
    where
        C: Into<Expr>,
        EA: Into<Expr>,
        EB: Into<Expr>,
    {
        Self {
            cond: Box::new(cond.into()),
            expr_a: Box::new(expr_a.into()),
            expr_b: Box::new(expr_b.into()),
        }
    }
}

impl Compilable for Cond {
    fn compile(&self) -> String {
        format!(
            "Either({}, {}, {})",
            self.cond.compile(),
            self.expr_a.compile(),
            self.expr_b.compile()
        )
    }
}

impl Into<Expr> for Cond {
    fn into(self) -> Expr {
        Expr::Cond(self)
    }
}

impl fmt::Debug for Cond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Cond({:?}, A({:?}), B({:?}))",
            self.cond, self.expr_a, self.expr_b
        )
    }
}
