use crate::ast::exprs::{Const, Ref, Table};
use crate::ast::{Stmt, Expr};
use crate::ast::stmts::Return;
use crate::compilation::{Compilable, Scope};
use std::fmt;

#[derive(Clone)]
pub struct Export {
    items: Vec<Expr>
}

impl Export {
    pub fn new<T>(items: Vec<T>) -> Self where T: Into<Expr> {
        Self {
            items: items.into_iter().map(|item| item.into()).collect()
        }
    }
}

impl Compilable for Export {
    fn compile(&self, scope: &Scope) -> String {
        let mut table = Table::new();

        for item in &self.items {
            table.insert(Const::from(item.compile(scope)), item);
        }

        Return::new(Some(table)).compile(scope)
    }
}

impl Into<Stmt> for Export {
    fn into(self) -> Stmt {
        Stmt::Export(self)
    }
}

impl fmt::Debug for Export {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Export({:?})", self.items)
    }
}
