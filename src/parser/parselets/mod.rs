use crate::ast::Expr;
use crate::lexer::Token;
use crate::parser::Parser;

pub mod led;
pub mod nud;

// Null-denotation rule
pub trait Nud {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expr, String>;
    fn get_precedence(&self) -> i8;
}

// Left-denotation rule
pub trait Led {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String>;
    fn get_precedence(&self) -> i8;
}


