use crate::parser::parselets::Led;
use crate::parser::Parser;
use crate::ast::Expr;
use crate::lexer::{Op, Token};
use crate::ast::exprs::Dot;

pub struct DotParselet;

impl Led for DotParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::Op(Op::Dot));

        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        Ok(Dot::new(lhs, rhs).into())
    }

    fn get_precedence(&self) -> i8 {
        15
    }
}