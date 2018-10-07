use std::fmt;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
pub enum Literal {
    Bool(bool),
    Float(f64),
    Integer(i64),
    Nil,
    String(String),
}

impl FromStr for Literal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Literal::String(s.to_owned()))
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Literal::Bool(value) => write!(f, "{:?}", value),
            &Literal::Float(ref value) => write!(f, "{:?}", value),
            &Literal::Integer(ref value) => write!(f, "{:?}", value),
            &Literal::Nil => write!(f, "nil"),
            &Literal::String(ref value) => write!(f, "{:?}", value)
        }
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Literal::Bool(value) => write!(f, "[literal (bool): {:?}]", value),
            &Literal::Float(ref value) => write!(f, "[literal (float): {:?}]", value),
            &Literal::Integer(ref value) => write!(f, "[literal (integer): {:?}]", value),
            &Literal::Nil => write!(f, "[literal (nil)]"),
            &Literal::String(ref value) => write!(f, "[literal (string): {:?}]", value)
        }
    }
}