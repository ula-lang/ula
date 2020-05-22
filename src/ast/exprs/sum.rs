use std::{fmt, io};
use std::borrow::Cow;

use ptree::{Style, TreeItem, TreeBuilder};

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};
use crate::debug::TreeNode;

#[derive(Clone)]
pub struct Sum {
    lhs: Box<Expr>,
    rhs: Box<Expr>
}

impl Sum {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into())
        }
    }
}

impl Into<Expr> for Sum {
    fn into(self) -> Expr {
        Expr::Sum(self)
    }
}

impl Compilable for Sum {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} + {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl fmt::Debug for Sum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sum({:?}, {:?})", self.lhs, self.rhs)
    }
}

impl TreeNode for Sum {
    fn write_tree(&self, builder: &mut TreeBuilder) {
        builder.begin_child("Sum".to_owned());
        self.lhs.write_tree(builder);
        self.rhs.write_tree(builder);
        builder.end_child();
    }
}