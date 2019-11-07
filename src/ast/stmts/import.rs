use crate::ast::exprs::{Const, FCall, Index, Ref};
use crate::ast::stmts::VarDecl;
use crate::compilation::{Compilable, Scope};
use std::fmt;
use crate::ast::Stmt;

#[derive(Clone)]
pub struct Import {
    idents: Vec<String>,
    source_file: String,
}

impl Import {
    pub fn new<S>(idents: Vec<String>, source_file: S) -> Self where S: ToString {
        Self {
            idents,
            source_file: source_file.to_string(),
        }
    }
}

impl Compilable for Import {
    fn compile(&self, scope: &Scope) -> String {
        let init_exprs = {
            let mut init_exprs = Vec::new();

            let include_call = FCall::new(
                Ref::new("include"),
                vec![
                    Const::from(format!("{}.lua", self.source_file)).into()
                ],
            );

            for ident in &self.idents {
                init_exprs.push(Index::new(include_call.clone(), Const::from(ident)).into());
            }

            init_exprs
        };

        VarDecl::new(self.idents.clone(), Some(init_exprs)).compile(scope)
    }
}

impl Into<Stmt> for Import {
    fn into(self) -> Stmt {
        Stmt::Import(self)
    }
}

impl fmt::Debug for Import {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Import({:?}, From({:?}))", self.idents, self.source_file)
    }
}