use std::fmt;

use ast::{Expr, Stmt};
use compilation::Compilable;

#[derive(Clone)]
pub struct ForEach {
    vars: (String, Option<String>),
    expr: Expr,
    body: Vec<Stmt>
}

impl ForEach {
    pub fn new<E>(vars: (String, Option<String>), expr: E, body: Vec<Stmt>) -> Self where E: Into<Expr> {
        Self {
            vars,
            expr: expr.into(),
            body
        }
    }
}

impl Compilable for ForEach {
    fn compile(&self) -> String {
        let mut compiled = String::new();

        compiled.push_str(&format!("for {}", self.vars.0));

        if let Some(ref var1) = self.vars.1 {
            compiled.push_str(&format!(", {}", var1));
        }

        compiled.push_str(&format!(" in {} do\r\n", self.expr.compile()));

        compiled.push_str(&self.body.compile_indented(1));

        compiled.push_str("\r\nend");

        compiled
    }
}

impl Into<Stmt> for ForEach {
    fn into(self) -> Stmt {
        Stmt::ForEach(self)
    }
}

impl fmt::Debug for ForEach {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ForEach(Vars({:?}), Expr({:?}), Body({:?}))", self.vars, self.expr, self.body)
    }
}