use crate::parser::parselets::Led;
use crate::parser::Parser;
use crate::ast::Expr;
use crate::lexer::{Op, Token};
use crate::ast::exprs::{Assignment, Sub, Sum};

pub struct PostfixParselet;

impl Led for PostfixParselet {
    fn parse(&self, _parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        match token {
            Token::Op(Op::Incr) => {
                Ok(Assignment::new(lhs.clone(), Sum::new(lhs, 1)).into())
            }

            Token::Op(Op::Decr) => {
                Ok(Assignment::new(lhs.clone(), Sub::new(lhs, 1)).into())
            }

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        15
    }
}