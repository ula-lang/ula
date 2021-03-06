use crate::ast::Expr;
use crate::ast::exprs::BOr;
use crate::lexer::{Op, Token};
use crate::parser::parselets::Led;
use crate::parser::Parser;

pub struct BOrParselet;

impl Led for BOrParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::Op(Op::BOr));

        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        Ok(BOr::new(lhs, rhs).into())
    }

    fn get_precedence(&self) -> i8 {
        5
    }
}