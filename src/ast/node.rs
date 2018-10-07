use std::fmt;

use ast::Stmt;
use ast::decls::*;
use compilation::Compilable;

pub enum Node {
    ClassDecl(ClassDecl),
    EnumDecl(EnumDecl),
    FuncDecl(FuncDecl),
    Stmt(Stmt)
}

impl Compilable for Node {
    fn compile(&self) -> String {
        match self {
            &Node::ClassDecl(ref decl) => decl.compile(),

            &Node::EnumDecl(ref decl) => decl.compile(),

            &Node::FuncDecl(ref decl) => decl.compile(),

            &Node::Stmt(ref stmt) => stmt.compile()
        }
    }
}

impl From<Stmt> for Node {
    fn from(stmt: Stmt) -> Self {
        Node::Stmt(stmt)
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node(")?;

        match self {
            &Node::ClassDecl(ref decl) => write!(f, "{:?}", decl)?,

            &Node::EnumDecl(ref decl) => write!(f, "{:?}", decl)?,

            &Node::FuncDecl(ref decl) => write!(f, "{:?}", decl)?,

            &Node::Stmt(ref stmt) => write!(f, "{:?}", stmt)?
        }

        write!(f, ")")
    }
}