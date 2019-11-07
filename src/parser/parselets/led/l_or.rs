use crate::ast::Expr;
use crate::ast::exprs::LOr;
use crate::lexer::{Op, Token};
use crate::parser::parselets::Led;
use crate::parser::Parser;

pub struct LOrParselet;

impl Led for LOrParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::Op(Op::LOr));

        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        Ok(LOr::new(lhs, rhs).into())
    }

    fn get_precedence(&self) -> i8 {
        3
    }
}