use parser::parselets::Led;
use parser::Parser;
use ast::Expr;
use lexer::{Op, Token};
use ast::exprs::{LShift, RShift};

pub struct ShiftParselet;

impl Led for ShiftParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        match token {
            Token::Op(Op::LShift) => Ok(LShift::new(lhs, rhs).into()),

            Token::Op(Op::RShift) => Ok(RShift::new(lhs, rhs).into()),

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        10
    }
}