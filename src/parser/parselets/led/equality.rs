use ast::Expr;
use ast::exprs::{Eq, Ne};
use lexer::{Op, Token};
use parser::parselets::Led;
use parser::Parser;

pub struct EqualityParselet;

impl Led for EqualityParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        match token {
            Token::Op(Op::Eq) => Ok(Eq::new(lhs, rhs).into()),

            Token::Op(Op::Ne) => Ok(Ne::new(lhs, rhs).into()),

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        9
    }
}