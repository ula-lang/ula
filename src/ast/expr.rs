use std::fmt;

use crate::ast::exprs::*;
use crate::compilation::{Compilable, Scope};

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub enum Expr {
    #[derivative(Debug="transparent")]
    Assignment(Assignment),
    #[derivative(Debug="transparent")]
    Await(Await),
    #[derivative(Debug="transparent")]
    Bracketed(Parens),
    #[derivative(Debug="transparent")]
    BAnd(BAnd),
    #[derivative(Debug="transparent")]
    BOr(BOr),
    #[derivative(Debug="transparent")]
    Coalesce(Coalesce),
    #[derivative(Debug="transparent")]
    Concat(Concat),
    #[derivative(Debug="transparent")]
    Cond(Cond),
    #[derivative(Debug="transparent")]
    Const(Const),
    #[derivative(Debug="transparent")]
    Dot(Dot),
    #[derivative(Debug="transparent")]
    Div(Div),
    #[derivative(Debug="transparent")]
    Eq(Eq),
    #[derivative(Debug="transparent")]
    FCall(FCall),
    #[derivative(Debug="transparent")]
    Gt(Gt),
    #[derivative(Debug="transparent")]
    GtEq(GtEq),
    #[derivative(Debug="transparent")]
    Index(Index),
    #[derivative(Debug="transparent")]
    Lambda(Lambda),
    #[derivative(Debug="transparent")]
    Len(Len),
    #[derivative(Debug="transparent")]
    LAnd(LAnd),
    #[derivative(Debug="transparent")]
    LOr(LOr),
    #[derivative(Debug="transparent")]
    LShift(LShift),
    #[derivative(Debug="transparent")]
    Lt(Lt),
    #[derivative(Debug="transparent")]
    LtEq(LtEq),
    #[derivative(Debug="transparent")]
    MCall(MCall),
    #[derivative(Debug="transparent")]
    Mod(Mod),
    #[derivative(Debug="transparent")]
    Mul(Mul),
    #[derivative(Debug="transparent")]
    Ne(Ne),
    #[derivative(Debug="transparent")]
    Neg(Neg),
    #[derivative(Debug="transparent")]
    New(New),
    None,
    #[derivative(Debug="transparent")]
    Not(Not),
    #[derivative(Debug="transparent")]
    RShift(RShift),
    #[derivative(Debug="transparent")]
    Sub(Sub),
    #[derivative(Debug="transparent")]
    Sum(Sum),
    #[derivative(Debug="transparent")]
    Table(Table),
    #[derivative(Debug="transparent")]
    Var(Ref),
    #[derivative(Debug="transparent")]
    Xor(Xor)
}

impl Compilable for Expr {
    fn compile(&self, scope: &Scope) -> String {
        match self {
            &Expr::Assignment(ref expr) => expr.compile(scope),

            &Expr::Await(ref expr) => expr.compile(scope),

            &Expr::Bracketed(ref expr) => expr.compile(scope),

            &Expr::BAnd(ref expr) => expr.compile(scope),

            &Expr::BOr(ref expr) => expr.compile(scope),

            &Expr::Coalesce(ref expr) => expr.compile(scope),

            &Expr::Concat(ref expr) => expr.compile(scope),

            &Expr::Cond(ref expr) => expr.compile(scope),

            &Expr::Const(ref expr) => expr.compile(scope),

            &Expr::Dot(ref expr) => expr.compile(scope),

            &Expr::Div(ref expr) => expr.compile(scope),

            &Expr::Eq(ref expr) => expr.compile(scope),

            &Expr::FCall(ref expr) => expr.compile(scope),

            &Expr::Gt(ref expr) => expr.compile(scope),

            &Expr::GtEq(ref expr) => expr.compile(scope),

            &Expr::Index(ref expr) => expr.compile(scope),

            &Expr::Lambda(ref expr) => expr.compile(scope),

            &Expr::Len(ref expr) => expr.compile(scope),

            &Expr::LAnd(ref expr) => expr.compile(scope),

            &Expr::LOr(ref expr) => expr.compile(scope),

            &Expr::LShift(ref expr) => expr.compile(scope),

            &Expr::Lt(ref expr) => expr.compile(scope),

            &Expr::LtEq(ref expr) => expr.compile(scope),

            &Expr::MCall(ref expr) => expr.compile(scope),

            &Expr::Mod(ref expr) => expr.compile(scope),

            &Expr::Mul(ref expr) => expr.compile(scope),

            &Expr::Ne(ref expr) => expr.compile(scope),

            &Expr::Neg(ref expr) => expr.compile(scope),

            &Expr::New(ref expr) => expr.compile(scope),

            &Expr::None => "".to_owned(),

            &Expr::Not(ref expr) => expr.compile(scope),

            &Expr::RShift(ref expr) => expr.compile(scope),

            &Expr::Sub(ref expr) => expr.compile(scope),

            &Expr::Sum(ref expr) => expr.compile(scope),

            &Expr::Table(ref expr) => expr.compile(scope),

            &Expr::Var(ref expr) => expr.compile(scope),

            &Expr::Xor(ref expr) => expr.compile(scope)
        }
    }
}

impl Compilable for Vec<Expr> {
    fn compile(&self, scope: &Scope) -> String {
        let mut compiled = String::new();

        for (i, expr) in self.iter().enumerate() {
            compiled.push_str(&expr.compile(scope));

            if i + 1 < self.len() {
                compiled.push_str(", ");
            }
        }

        compiled
    }
}

impl From<bool> for Expr {
    fn from(value: bool) -> Self {
        Expr::Const(value.into())
    }
}

impl From<i64> for Expr {
    fn from(value: i64) -> Self {
        Expr::Const(value.into())
    }
}

impl From<f64> for Expr {
    fn from(value: f64) -> Self {
        Expr::Const(value.into())
    }
}

impl Compilable for Option<Expr> {
    fn compile(&self, scope: &Scope) -> String {
        match self {
            None => &Expr::None,

            Some(ref expr) => expr
        }.compile(scope)
    }
}
