use std::fmt;

use ast::exprs::*;
use compilation::Compilable;

#[derive(Clone)]
pub enum Expr {
    Assignment(Assignment),
    Await(Await),
    Bracketed(Parens),
    BAnd(BAnd),
    BOr(BOr),
    Concat(Concat),
    Cond(Cond),
    Const(Const),
    Dot(Dot),
    Div(Div),
    Eq(Eq),
    FCall(FCall),
    Gt(Gt),
    GtEq(GtEq),
    Index(Index),
    Lambda(Lambda),
    Len(Len),
    LAnd(LAnd),
    LOr(LOr),
    LShift(LShift),
    Lt(Lt),
    LtEq(LtEq),
    MCall(MCall),
    Mod(Mod),
    Mul(Mul),
    Ne(Ne),
    Neg(Neg),
    New(New),
    None,
    Not(Not),
    RShift(RShift),
    Sub(Sub),
    Sum(Sum),
    Table(Table),
    Var(Ref),
    Xor(Xor)
}

impl Compilable for Expr {
    fn compile(&self) -> String {
        match self {
            &Expr::Assignment(ref expr) => expr.compile(),

            &Expr::Await(ref expr) => expr.compile(),

            &Expr::Bracketed(ref expr) => expr.compile(),

            &Expr::BAnd(ref expr) => expr.compile(),

            &Expr::BOr(ref expr) => expr.compile(),

            &Expr::Concat(ref expr) => expr.compile(),

            &Expr::Cond(ref expr) => expr.compile(),

            &Expr::Const(ref expr) => expr.compile(),

            &Expr::Dot(ref expr) => expr.compile(),

            &Expr::Div(ref expr) => expr.compile(),

            &Expr::Eq(ref expr) => expr.compile(),

            &Expr::FCall(ref expr) => expr.compile(),

            &Expr::Gt(ref expr) => expr.compile(),

            &Expr::GtEq(ref expr) => expr.compile(),

            &Expr::Index(ref expr) => expr.compile(),

            &Expr::Lambda(ref expr) => expr.compile(),

            &Expr::Len(ref expr) => expr.compile(),

            &Expr::LAnd(ref expr) => expr.compile(),

            &Expr::LOr(ref expr) => expr.compile(),

            &Expr::LShift(ref expr) => expr.compile(),

            &Expr::Lt(ref expr) => expr.compile(),

            &Expr::LtEq(ref expr) => expr.compile(),

            &Expr::MCall(ref expr) => expr.compile(),

            &Expr::Mod(ref expr) => expr.compile(),

            &Expr::Mul(ref expr) => expr.compile(),

            &Expr::Ne(ref expr) => expr.compile(),

            &Expr::Neg(ref expr) => expr.compile(),

            &Expr::New(ref expr) => expr.compile(),

            &Expr::None => "".to_owned(),

            &Expr::Not(ref expr) => expr.compile(),

            &Expr::RShift(ref expr) => expr.compile(),

            &Expr::Sub(ref expr) => expr.compile(),

            &Expr::Sum(ref expr) => expr.compile(),

            &Expr::Table(ref expr) => expr.compile(),

            &Expr::Var(ref expr) => expr.compile(),

            &Expr::Xor(ref expr) => expr.compile()
        }
    }
}

impl Compilable for Vec<Expr> {
    fn compile(&self) -> String {
        let mut compiled = String::new();

        for (i, expr) in self.iter().enumerate() {
            compiled.push_str(&expr.compile());

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
    fn compile(&self) -> String {
        match self {
            None => &Expr::None,

            Some(ref expr) => expr
        }.compile()
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Expr(")?;

        match self {
            &Expr::Assignment(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Await(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Bracketed(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::BAnd(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::BOr(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Concat(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Cond(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Const(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Dot(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Div(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Eq(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::FCall(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Gt(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::GtEq(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Index(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Lambda(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Len(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::LAnd(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::LOr(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::LShift(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Lt(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::LtEq(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::MCall(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Mod(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Mul(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Ne(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::New(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Neg(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::None => write!(f, "None")?,

            &Expr::Not(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::RShift(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Sub(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Sum(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Table(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Var(ref expr) => write!(f, "{:?}", expr)?,

            &Expr::Xor(ref expr) => write!(f, "{:?}", expr)?
        }

        write!(f, ")")
    }
}
