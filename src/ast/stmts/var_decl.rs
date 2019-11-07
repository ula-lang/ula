use std::fmt;

use crate::ast::{Expr, Stmt};
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct VarDecl {
    idents: Vec<String>,
    init_exprs: Option<Vec<Expr>>
}

impl VarDecl {
    pub fn new(idents: Vec<String>, init_exprs: Option<Vec<Expr>>) -> Self {
        Self {
            idents,
            init_exprs
        }
    }
}

impl Compilable for VarDecl {
    fn compile(&self, scope: &Scope) -> String {
        self.idents.iter().for_each(|ident| scope.add_variable(ident));

        let mut compiled = format!("local {}", self.idents.join(", "));

        if let Some(ref init_exprs) = self.init_exprs {
            compiled.push_str(&format!(" = {}", init_exprs.compile(scope)));
        }

        compiled.push(';');

        compiled
    }
}

impl Into<Stmt> for VarDecl {
    fn into(self) -> Stmt {
        Stmt::VarDecl(self)
    }
}

impl fmt::Debug for VarDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VarDecl(Idents({:?})", self.idents)?;

        if let Some(ref init_exprs) = self.init_exprs {
            write!(f, ", InitExprs({:?})", init_exprs)?
        }

        write!(f, ")")
    }
}