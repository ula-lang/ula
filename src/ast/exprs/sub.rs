use std::{fmt, io};

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};
use ptree::{TreeItem, Style};
use std::borrow::Cow;

#[derive(Clone)]
pub struct Sub {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

impl Sub {
    pub fn new<EL: Into<Expr>, ER: Into<Expr>>(lhs: EL, rhs: ER) -> Self {
        Self {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into()),
        }
    }
}

impl Into<Expr> for Sub {
    fn into(self) -> Expr {
        Expr::Sub(self)
    }
}

impl Compilable for Sub {
    fn compile(&self, scope: &Scope) -> String {
        format!("({} - {})", self.lhs.compile(scope), self.rhs.compile(scope))
    }
}

impl fmt::Debug for Sub {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sub({:?}, {:?})", self.lhs, self.rhs)
    }
}

// impl TreeItem for Sub {
//     type Child = Expr;
//
//     fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
//         write!(f, "{}", style.paint("sub"))
//     }
//
//     fn children(&self) -> Cow<[Self::Child]> {
//         Cow::from(vec![*self.lhs.clone(), *self.rhs.clone()])
//     }
// }
