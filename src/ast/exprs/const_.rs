use std::{fmt, io};
use std::borrow::Cow;
use std::io::Error;

use ptree::{Style, TreeBuilder, TreeItem};

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};
use crate::debug::TreeNode;

#[derive(Clone)]
pub enum Const {
    Bool(bool),
    Int(i64),
    Float(f64),
    Nil,
    String(String),
}

impl Compilable for Const {
    fn compile(&self, _scope: &Scope) -> String {
        match self {
            &Const::Bool(value) => value.to_string(),

            &Const::Int(value) => value.to_string(),

            &Const::Float(value) => value.to_string(),

            &Const::Nil => "nil".to_owned(),

            &Const::String(ref value) => format!(r#""{}""#, value),
        }
    }
}

impl Into<Expr> for Const {
    fn into(self) -> Expr {
        Expr::Const(self)
    }
}

impl From<bool> for Const {
    fn from(value: bool) -> Self {
        Const::Bool(value)
    }
}

impl From<i64> for Const {
    fn from(value: i64) -> Self {
        Const::Int(value)
    }
}

impl From<f64> for Const {
    fn from(value: f64) -> Self {
        Const::Float(value)
    }
}

impl<'a> From<&'a str> for Const {
    fn from(value: &'a str) -> Self {
        value.to_owned().into()
    }
}

impl From<String> for Const {
    fn from(value: String) -> Self {
        Const::String(value)
    }
}

impl<'a> From<&'a String> for Const {
    fn from(value: &'a String) -> Self {
        value.to_owned().into()
    }
}

impl fmt::Debug for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Const(")?;

        match self {
            &Const::Bool(value) => write!(f, "Bool({})", value)?,

            &Const::Int(value) => write!(f, "Int({})", value)?,

            &Const::Float(value) => write!(f, "Float({})", value)?,

            &Const::Nil => write!(f, "Nil")?,

            &Const::String(ref value) => write!(f, "String({:?})", value)?
        }

        write!(f, ")")
    }
}

impl TreeNode for Const {
    fn write_tree(&self, builder: &mut TreeBuilder) {
        builder.add_empty_child(format!("{:?}", self));
    }
}
