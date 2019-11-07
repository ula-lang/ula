use crate::ast::decls::FuncDecl;
use crate::ast::Expr;
use crate::ast::stmts::*;
use crate::compilation::{Compilable, Scope};
use std::fmt;

#[derive(Clone)]
pub enum Stmt {
    Break,
    Export(Export),
    Expr(Expr),
    For(For),
    ForEach(ForEach),
    // A `FuncDecl` be a `Stmt` or a `Node`
    FuncDecl(FuncDecl),
    IfElse(IfElse),
    Import(Import),
    VarDecl(VarDecl),
    Return(Return),
    SwitchCase(SwitchCase),
    While(While),
    Yield(Yield),
}

impl Compilable for Stmt {
    fn compile(&self, scope: &Scope) -> String {
        match self {
            &Stmt::Break => {
                "break;".to_owned()
            }

            &Stmt::Export(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }

            &Stmt::Expr(ref expr) => {
                format!("{};", expr.compile(scope))
            }

            &Stmt::For(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }

            &Stmt::ForEach(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }

            &Stmt::FuncDecl(ref decl) => {
                format!("{}", decl.compile(scope))
            }

            &Stmt::IfElse(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }

            &Stmt::Import(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }

            &Stmt::VarDecl(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }

            &Stmt::Return(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }

            &Stmt::SwitchCase(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }

            &Stmt::While(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }

            &Stmt::Yield(ref stmt) => {
                format!("{}", stmt.compile(scope))
            }
        }
    }
}

impl<T> From<T> for Stmt where T: Into<Expr> {
    fn from(expr: T) -> Self {
        Stmt::Expr(expr.into())
    }
}

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stmt(")?;

        match self {
            &Stmt::Break => {
                write!(f, "Break")?
            }

            &Stmt::Export(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }

            &Stmt::Expr(ref expr) => {
                write!(f, "{:?}", expr)?
            }

            &Stmt::For(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }

            &Stmt::ForEach(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }

            &Stmt::FuncDecl(ref decl) => {
                write!(f, "{:?}", decl)?
            }

            &Stmt::IfElse(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }

            &Stmt::Import(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }

            &Stmt::VarDecl(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }

            &Stmt::Return(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }

            &Stmt::SwitchCase(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }

            &Stmt::While(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }

            &Stmt::Yield(ref stmt) => {
                write!(f, "{:?}", stmt)?
            }
        }

        write!(f, ")")
    }
}