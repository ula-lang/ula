use crate::ast::{Expr, Node, Stmt};
use crate::ast::exprs::{Assignment, Coalesce, Cond, Const, Dot, Eq, FCall, Lambda, Ref};
use crate::ast::stmts::{Import, Return};
use crate::compilation::{Compilable, Scope};
use std::fmt;
use crate::ast::stmts::IfElse;

#[derive(Clone)]
pub struct FuncDecl {
    // 0: Local, 1: Static, 2: Async
    flags: (bool, bool, bool),
    pub ident: String,
    param_idents: Vec<String>,
    body: Vec<Stmt>,
}

impl FuncDecl {
    pub fn new<I>(
        flags: (bool, bool, bool),
        ident: I,
        params: Vec<(String, Option<Expr>)>,
        mut body: Vec<Stmt>,
    ) -> Self
        where
            I: ToString
    {
        let mut param_idents = Vec::new();

        let mut i = 0;

        for (ident, maybe_default) in params {
            if let Some(default) = maybe_default {
                let ident_ref = Ref::new(ident.clone());

                body.insert(
                    i,
                    IfElse::new(
                        Eq::new(ident_ref.clone(), Const::Nil),
                        vec![
                            Stmt::Expr(Assignment::new(vec![ident_ref.clone()], vec![default]).into())
                        ],
                        None
                    ).into()
                );

                i += 1;
            }

            param_idents.push(ident);
        }

        Self {
            flags,
            ident: ident.to_string(),
            param_idents,
            body,
        }
    }

    pub fn body_mut(&mut self) -> &mut Vec<Stmt> {
        &mut self.body
    }

    #[inline(always)]
    pub fn is_local(&self) -> bool {
        self.flags.0
    }

    #[inline(always)]
    pub fn is_static(&self) -> bool {
        self.flags.1
    }

    #[inline(always)]
    pub fn is_async(&self) -> bool {
        self.flags.2
    }
}

impl Compilable for FuncDecl {
    fn compile(&self, scope: &Scope) -> String {
        scope.add_function(&self.ident);

        let mut compiled = String::new();

        if self.is_local() {
            compiled.push_str("local ");
        }

        compiled.push_str(&format!("function {}(", self.ident));

        for (i, ident) in self.param_idents.iter().enumerate() {
            compiled.push_str(ident);

            if i + 1 < self.param_idents.len() {
                compiled.push_str(", ");
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
                                Lambda::new((false, ), Vec::new(), self.body.clone()).into()
                            ]),
                        )
                    )
                ).into()
            ];

            compiled.push_str(&async_body.compile_indented(scope, 1));
        } else {
            compiled.push_str(&self.body.compile_indented(scope, 1));
        }

        compiled.push_str("\r\nend");

        compiled
    }
}

impl Into<Node> for FuncDecl {
    fn into(self) -> Node {
        Node::FuncDecl(self)
    }
}

impl Into<Stmt> for FuncDecl {
    fn into(self) -> Stmt {
        Stmt::FuncDecl(self)
    }
}

impl fmt::Debug for FuncDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "FuncDecl(Flags({:?}), Ident({:?}), ",
            self.flags, self.ident
        )?;

        if self.param_idents.len() > 0 {
            write!(f, "Params({:?}), ", self.param_idents)?;
        }

        write!(f, "Body({:?}))", self.body)
    }
}
