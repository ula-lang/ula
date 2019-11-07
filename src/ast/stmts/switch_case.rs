use std::fmt;

use crate::ast::{Expr, Stmt};
use crate::ast::exprs::{Assignment, Const, Eq, LOr, Ref};
use crate::ast::stmts::{IfElse, VarDecl};
use crate::ast::util::{random_unique_idents, random_unique_ident};
use crate::compilation::{Compilable, Scope};

#[derive(Clone, Debug)]
pub struct SwitchCase {
    term: Expr,
    cases: Vec<(Expr, Vec<Stmt>)>,
}

impl SwitchCase {
    pub fn new<E>(term: E, cases: Vec<(Expr, Vec<Stmt>)>) -> Self where E: Into<Expr> {
        Self {
            term: term.into(),
            cases,
        }
    }
}

impl Compilable for SwitchCase {
    fn compile(&self, scope: &Scope) -> String {
        let scope = &scope.create_child();

        let mut compiled = String::new();

        compiled.push_str("do\n");

        // TEMP
        let end_label_ident = random_unique_ident(scope);

        scope.add_variable(&end_label_ident);
        // END TEMP

        let (term_ref, matched_ref) = {
            let (term_ident, matched_ident) = {
                let mut iter = random_unique_idents(scope);

                (iter.next().unwrap(), iter.next().unwrap())
            };

            let decl = VarDecl::new(
                vec![term_ident.clone(), matched_ident.clone()],
                Some(vec![self.term.clone(), Const::Bool(false).into()]),
            );

            compiled.push_str(&decl.compile_indented(scope, 1));

            compiled.push('\n');

            (Ref::new(term_ident), Ref::new(matched_ident))
        };

        let matched_assign = Stmt::Expr(
            Assignment::new(matched_ref.clone(), Const::Bool(true)).into()
        );

        // TEMP
        // Fake goto using hack
        let break_stmt: Stmt = Ref::new(format!("goto {}", end_label_ident)).into();
        // END TEMP

        for (cond, body) in self.cases.iter().cloned() {
            let cond = LOr::new(
                matched_ref.clone(),
                Eq::new(term_ref.clone(), cond),
            );

            let mut body: Vec<Stmt> = body.into_iter()
                .map(|stmt| match stmt {
                    Stmt::Break => break_stmt.clone(),

                    _ => stmt
                })
                .collect();

            body.push(matched_assign.clone());

            let if_else = IfElse::new(cond, body, None);

            compiled.push_str(&if_else.compile_indented(scope, 1));

            compiled.push('\n');
        }

        // TEMP
        compiled.push_str(&format!("    ::{}::\n", end_label_ident));
        // END TEMP

        compiled.push_str("end");

        compiled
    }
}

impl Into<Stmt> for SwitchCase {
    fn into(self) -> Stmt {
        Stmt::SwitchCase(self)
    }
}

