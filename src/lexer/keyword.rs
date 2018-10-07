use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub enum Keyword {
    Async,
    Await,
    Class,
    Else,
    Enum,
    Extends,
    Export,
    For,
    ForEach,
    From,
    Function,
    If,
    Import,
    In,
    Var,
    Local,
    New,
    Return,
    Static,
    While,
    Yield
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'")?;
        
        match self {
            &Keyword::Async => write!(f, "async"),
            &Keyword::Await => write!(f, "await"),
            &Keyword::Class => write!(f, "class"),
            &Keyword::Else => write!(f, "else"),
            &Keyword::Enum => write!(f, "enum"),
            &Keyword::Export => write!(f, "export"),
            &Keyword::Extends => write!(f, "extends"),
            &Keyword::For => write!(f, "for"),
            &Keyword::ForEach => write!(f, "foreach"),
            &Keyword::From => write!(f, "from"),
            &Keyword::Function => write!(f, "function"),
            &Keyword::If => write!(f, "if"),
            &Keyword::Import => write!(f, "import"),
            &Keyword::In => write!(f, "in"),
            &Keyword::Var => write!(f, "var"),
            &Keyword::Local => write!(f, "local"),
            &Keyword::New => write!(f, "new"),
            &Keyword::Return => write!(f, "return"),
            &Keyword::Static => write!(f, "static"),
            &Keyword::While => write!(f, "while"),
            &Keyword::Yield => write!(f, "yield")
        }?;

        write!(f, "'")
    }
}

impl fmt::Debug for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Keyword({})", self)
    }
}