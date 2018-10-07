use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Op {
    // Assignment
    Assign,
    AddAssign,
    BAndAssign,
    BOrAssign,
    Decr,
    DivAssign,
    Incr,
    LShiftAssign,
    ModAssign,
    MulAssign,
    RShiftAssign,
    SubAssign,
    XorAssign,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Comparison
    Eq,
    Ne,
    Gt,
    Lt,
    GtEq,
    LtEq,

    // Logical
    LAnd,
    LOr,
    Not,

    // Bitwise
    BAnd,
    BOr,
    Xor,
    LShift,
    RShift,

    // Misc
    Concat,
    Cond,
    Dot,
    Ellipsis,
    Interp,
    Len
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'")?;

        match self {
            // Assignment
            &Op::Assign => write!(f, "="),
            &Op::AddAssign => write!(f, "+="),
            &Op::BAndAssign => write!(f, "&="),
            &Op::BOrAssign => write!(f, "|="),
            &Op::Decr => write!(f, "--"),
            &Op::DivAssign => write!(f, "/="),
            &Op::Incr => write!(f, "++"),
            &Op::LShiftAssign => write!(f, "<<="),
            &Op::ModAssign => write!(f, "%="),
            &Op::MulAssign => write!(f, "*="),
            &Op::RShiftAssign => write!(f, ">>="),
            &Op::SubAssign => write!(f, "-="),
            &Op::XorAssign => write!(f, "^="),

            // Arithmetic
            &Op::Div => write!(f, "/"),
            &Op::Mod => write!(f, "%"),
            &Op::Mul => write!(f, "*"),
            &Op::Sub => write!(f, "-"),
            &Op::Add => write!(f, "+"),

            // Comparison
            &Op::Eq => write!(f, "=="),
            &Op::Ne => write!(f, "!="),
            &Op::Gt => write!(f, ">"),
            &Op::Lt => write!(f, "<"),
            &Op::GtEq => write!(f, ">="),
            &Op::LtEq => write!(f, "<="),

            // Logical
            &Op::LAnd => write!(f, "&&"),
            &Op::LOr => write!(f, "||"),
            &Op::Not => write!(f, "!"),

            // Bitwise
            &Op::BAnd => write!(f, "&"),
            &Op::BOr => write!(f, "|"),
            &Op::Xor => write!(f, "^"),
            &Op::LShift => write!(f, "<<"),
            &Op::RShift => write!(f, ">>"),

            // Misc
            &Op::Concat => write!(f, ".."),
            &Op::Cond => write!(f, "?"),
            &Op::Dot => write!(f, "."),
            &Op::Ellipsis => write!(f, "..."),
            &Op::Interp => write!(f, "$"),
            &Op::Len => write!(f, "#"),
        }?;

        write!(f, "'")
    }
}

impl fmt::Debug for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[op: {}]", self)
    }
}