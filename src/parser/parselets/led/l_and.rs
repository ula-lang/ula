use crate::ast::Expr;
use crate::ast::exprs::LAnd;
use crate::lexer::{Op, Token};
use crate::parser::parselets::Led;
use crate::parser::Parser;

pub struct LAndParselet;

impl Led for LAndParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::Op(Op::LAnd));

        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        Ok(LAnd::new(lhs, rhs).into())
    }

    fn get_precedence(&self) -> i8 {
        4
    }
}