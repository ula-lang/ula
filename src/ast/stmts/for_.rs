use std::fmt;

use crate::ast::{Expr, Stmt};
use crate::compilation::{Compilable, Scope};
use crate::ast::stmts::While;

#[derive(Clone)]
pub struct For {
    init: Box<Stmt>,
    while_stmt: While
}

impl For {
    pub fn new<IN, C, IT>(init: IN, cond: C, iter: IT, mut body: Vec<Stmt>) -> Self where IN: Into<Stmt>, C: Into<Expr>, IT: Into<Expr> {
        body.push(Stmt::Expr(iter.into()));
        let while_stmt = While::new(cond, body);

        Self {
            init: Box::new(init.into()),
            while_stmt
        }
    }
}

impl Compilable for For {
    fn compile(&self, scope: &Scope) -> String {
        let scope = &scope.create_child();

        let mut compiled = String::new();

        compiled.push_str("do\r\n");

        compiled.push_str(&self.init.compile_indented(scope, 1));

        compiled.push_str("\r\n");

        compiled.push_str(&self.while_stmt.compile_indented(scope, 1));

        compiled.push_str("\r\nend");

        compiled
    }
}

impl Into<Stmt> for For {
    fn into(self) -> Stmt {
        Stmt::For(self)
    }
}

impl fmt::Debug for For {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "For(Init({:?}), {:?})", self.init, self.while_stmt)
    }
}