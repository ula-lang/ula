use ast::Expr;
use ast::exprs::{Gt, GtEq, Lt, LtEq};
use lexer::{Op, Token};
use parser::parselets::Led;
use parser::Parser;

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