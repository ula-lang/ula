use crate::ast::Expr;
use crate::ast::exprs::Xor;
use crate::lexer::{Op, Token};
use crate::parser::parselets::Led;
use crate::parser::Parser;

pub struct XorParselet;

impl Led for XorParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::Op(Op::Xor));

        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        Ok(Xor::new(lhs, rhs).into())
    }

    fn get_precedence(&self) -> i8 {
        6
    }
}