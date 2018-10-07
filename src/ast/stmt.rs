use ast::decls::FuncDecl;
use ast::Expr;
use ast::stmts::*;
use compilation::Compilable;
use std::fmt;

#[derive(Clone)]
pub enum Stmt {
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
    While(While),
    Yield(Yield)
}

impl Compilable for Stmt {
    fn compile(&self) -> String {
        match self {
            &Stmt::Export(ref stmt) => {
                format!("{}", stmt.compile())
            }

            &Stmt::Expr(ref expr) => {
                format!("{};", expr.compile())
            }

            &Stmt::For(ref stmt) => {
                format!("{}", stmt.compile())
            }

            &Stmt::ForEach(ref stmt) => {
                format!("{}", stmt.compile())
            }

            &Stmt::FuncDecl(ref decl) => {
                format!("{}", decl.compile())
            }

            &Stmt::IfElse(ref stmt) => {
                format!("{}", stmt.compile())
            }

            &Stmt::Import(ref stmt) => {
                format!("{}", stmt.compile())
            }

            &Stmt::VarDecl(ref stmt) => {
                format!("{}", stmt.compile())
            }

            &Stmt::Return(ref stmt) => {
                format!("{}", stmt.compile())
            }

            &Stmt::While(ref stmt) => {
                format!("{}", stmt.compile())
            }

            &Stmt::Yield(ref stmt) => {
                format!("{}", stmt.compile())
            }
        }
    }
}

impl From<Expr> for Stmt {
    fn from(expr: Expr) -> Self {
        Stmt::Expr(expr)
    }
}

impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stmt(")?;

        match self {
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