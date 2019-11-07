use crate::ast::Expr;
use crate::ast::exprs::{Gt, GtEq, Lt, LtEq};
use crate::lexer::{Op, Token};
use crate::parser::parselets::Led;
use crate::parser::Parser;

pub struct RelationalParselet;

impl Led for RelationalParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        match token {
            Token::Op(Op::Gt) => Ok(Gt::new(lhs, rhs).into()),

            Token::Op(Op::Lt) => Ok(Lt::new(lhs, rhs).into()),

            Token::Op(Op::GtEq) => Ok(GtEq::new(lhs, rhs).into()),

            Token::Op(Op::LtEq) => Ok(LtEq::new(lhs, rhs).into()),

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        9
    }
}