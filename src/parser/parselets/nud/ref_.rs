use ast::Expr;
use ast::exprs::Ref;
use lexer::{Op, Token};
use parser::parselets::Nud;
use parser::Parser;

pub struct RefParselet;

impl Nud for RefParselet {
    fn parse(&self, _parser: &mut Parser, token: Token) -> Result<Expr, String> {
        match token {
            Token::Ident(ident) => {
                Ok(Ref::new(ident).into())
            }

            Token::Op(Op::Ellipsis) => {
                Ok(Ref::new("...".to_owned()).into())
            }

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        15
    }
}