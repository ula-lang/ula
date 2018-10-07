use std::fmt;

use ast::{Expr, Stmt};
use compilation::Compilable;
use ast::exprs::{Assignment, Cond, Const, Dot, Eq, FCall, Ref};
use ast::stmts::{Import, Return};

#[derive(Clone)]
pub struct Lambda {
    // 0: Async
    flags: (bool, ),
    param_idents: Vec<String>,
    body: Vec<Stmt>,
}

impl Lambda {
    pub fn new(flags: (bool, ), params: Vec<(String, Option<Expr>)>, mut body: Vec<Stmt>) -> Self {
        let mut param_idents = Vec::new();

        let mut i = 0;

        for (ident, maybe_default) in params {
            let ident = ident.to_string();

            param_idents.push(ident.clone());

            if let Some(default) = maybe_default {
                let ident_var = Ref::new(ident);

                body.insert(
                    i,
                    Stmt::Expr(
                        Assignment::new(
                            ident_var.clone(),
                            Cond::new(Eq::new(ident_var.clone(), Const::Nil), default, ident_var),
                        ).into(),
                    ),
                );

                i += 1;
            }
        }

        Self {
            flags,
            param_idents,
            body,
        }
    }

    #[inline(always)]
    pub fn is_async(&self) -> bool {
        self.flags.0
    }
}

impl Compilable for Lambda {
    fn compile(&self) -> String {
        let mut compiled = String::new();

        compiled.push_str("function (");

        for (i, ident) in self.param_idents.iter().enumerate() {
            compiled.push_str(ident);

            if i + 1 < self.param_idents.len() {
                compiled.push_str(", ")
            }
        }

        compiled.push_str(")\r\n");

        if self.is_async() {
            let async_body: Vec<Stmt> = vec![
                Import::new(
                    vec![
                        "Task".to_owned()
                    ],
                    "std/async/task",
                ).into(),
                Return::new(
                    Some(
                        Dot::new(
                            Ref::new("Task"),
                            FCall::new(Ref::new("Run"), vec![
                                Self::new((false, ), Vec::new(), self.body.clone()).into()
                            ]),
                        )
                    )
                ).into()
            ];

            compiled.push_str(&async_body.compile_indented(1));
        } else {
            compiled.push_str(&self.body.compile_indented(1));
        }

        compiled.push_str("\r\nend");

        compiled
    }
}

impl Into<Expr> for Lambda {
    fn into(self) -> Expr {
        Expr::Lambda(self)
    }
}

impl fmt::Debug for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Open lambda
        write!(f, "Lambda(")?;

        // Check for params
        if !self.param_idents.is_empty() {
            // Write params
            write!(f, "Params({:?}), ", self.param_idents)?;
        }

        // Write body
        write!(f, "Body({:?})", self.body)?;

        // Close lambda
        write!(f, ")")
    }
}