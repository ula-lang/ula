use std::fmt;

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct Ref {
    ident: String
}

impl Ref {
    pub fn new<I>(ident: I) -> Self where I: ToString {
        Self {
            ident: ident.to_string()
        }
    }

    pub fn ident(&self) -> &String {
        &self.ident
    }
}

impl Compilable for Ref {
    fn compile(&self, _scope: &Scope) -> String {
        format!("{}", self.ident)
    }
}

impl Into<Expr> for Ref {
    fn into(self) -> Expr {
        Expr::Var(self)
    }
}

impl From<String> for Ref {
    fn from(ident: String) -> Self {
        Self::new(ident)
    }
}

impl fmt::Debug for Ref {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Var({:?})", self.ident)
    }
}
