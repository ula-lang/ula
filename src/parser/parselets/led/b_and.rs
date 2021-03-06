use crate::ast::Expr;
use crate::ast::exprs::BAnd;
use crate::lexer::{Op, Token};
use crate::parser::parselets::Led;
use crate::parser::Parser;

pub struct BAndParselet;

impl Led for BAndParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::Op(Op::BAnd));

        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        Ok(BAnd::new(lhs, rhs).into())
    }

    fn get_precedence(&self) -> i8 {
        7
    }
}