use super::{Keyword, Literal, Op};

use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Token {
    Colon,
    Comment(String),
    Comma,
    Ident(String),
    Keyword(Keyword),
    LBrace,
    LBracket,
    Literal(Literal),
    LParens,
    Op(Op),
    RBrace,
    RBracket,
    RParens,
    Semicolon,
    Tilda,
    Whitespace(usize)
}

impl From<Keyword> for Token {
    fn from(keyword: Keyword) -> Self {
        Token::Keyword(keyword)
    }
}

impl From<Literal> for Token {
    fn from(literal: Literal) -> Self {
        Token::Literal(literal)
    }
}

impl From<Op> for Token {
    fn from(op: Op) -> Self {
        Token::Op(op)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Token::Colon => write!(f, "':'"),
            &Token::Comment(ref comment) => write!(f, "{:?}", comment),
            &Token::Comma => write!(f, "','"),
            &Token::Ident(ref ident) => write!(f, "{:?}", ident),
            &Token::Keyword(ref keyword) => write!(f, "{}", keyword),
            &Token::LBrace => write!(f, "'{{'"),
            &Token::LBracket => write!(f, "'['"),
            &Token::Literal(ref literal) => write!(f, "{}", literal),
            &Token::LParens => write!(f, "'('"),
            &Token::Op(ref op) => write!(f, "{}", op),
            &Token::RBrace => write!(f, "'}}'"),
            &Token::RBracket => write!(f, "']'"),
            &Token::RParens => write!(f, "')'"),
            &Token::Semicolon => write!(f, "';'"),
            &Token::Tilda => write!(f, "'~'"),
            &Token::Whitespace(_) => write!(f, "{:?}", self),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token(")?;

        match self {
            | &Token::Colon
            | &Token::Comma
            | &Token::LBrace
            | &Token::LBracket
            | &Token::LParens
            | &Token::RBrace
            | &Token::RBracket
            | &Token::RParens
            | &Token::Semicolon
            | &Token::Tilda
              => write!(f, "'{}'", self),

            &Token::Comment(ref comment) => write!(f, "[comment: {:?}]", comment),
            &Token::Ident(ref ident) => write!(f, "[ident: {:?}]", ident),
            &Token::Keyword(ref keyword) => write!(f, "{:?}", keyword),
            &Token::Literal(ref literal) => write!(f, "{:?}", literal),
            &Token::Op(ref op) => write!(f, "{:?}", op),
            &Token::Whitespace(lines) => {
                if lines > 0 {
                    if lines == 1 {
                        write!(f, "[newline]")
                    } else {
                        write!(f, "[newlines: {}]", lines)
                    }
                } else {
                    write!(f, "[whitespace]")
                }
            }
        }?;

        write!(f, ")")
    }
}

