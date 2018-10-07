use ast::exprs::{Const, Ref, Table};
use ast::Stmt;
use ast::stmts::Return;
use compilation::Compilable;
use std::fmt;

#[derive(Clone)]
pub struct Export {
    idents: Vec<String>
}

impl Export {
    pub fn new(idents: Vec<String>) -> Self {
        Self {
            idents
        }
    }
}

impl Compilable for Export {
    fn compile(&self) -> String {
        let mut table = Table::new();

        for ident in &self.idents {
            table.insert(Const::from(ident), Ref::new(ident));
        }

        Return::new(Some(table)).compile()
    }
}

impl Into<Stmt> for Export {
    fn into(self) -> Stmt {
        Stmt::Export(self)
    }
}

impl fmt::Debug for Export {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Export({:?})", self.idents)
    }
}
