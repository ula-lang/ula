use std::fmt;

use crate::ast::Expr;
use crate::ast::exprs::{Cond, Const, Eq, FCall, Lambda, Ref};
use crate::ast::stmts::{Return, VarDecl};
use crate::ast::util::random_unique_ident;
use crate::compilation::{Compilable, Scope};

#[derive(Clone, Debug)]
pub struct Coalesce {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

impl Coalesce {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into()),
        }
    }
}

impl Into<Expr> for Coalesce {
    fn into(self) -> Expr {
        Expr::Coalesce(self)
    }
}

impl Compilable for Coalesce {
    fn compile(&self, scope: &Scope) -> String {
        let temp_var_ident = random_unique_ident(scope);

        FCall::new(
            Lambda::new(
                (false, ),
                Vec::new(),
                vec![
                    VarDecl::new(
                        vec![temp_var_ident.clone()],
                        Some(vec![*self.lhs.clone()])
                    ).into(),
                    Return::new(
                        Cond::new(
                            Eq::new(Ref::new(temp_var_ident.clone()), Const::Nil),
                            *self.rhs.clone(),
                            Ref::new(temp_var_ident),
                        ).into()
                    ).into()
                ]
            ),
            Vec::new()
        ).compile(scope)
    }
}
