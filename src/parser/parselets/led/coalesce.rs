use crate::ast::Expr;
use crate::ast::exprs::Coalesce;
use crate::lexer::{Op, Token};
use crate::parser::parselets::Led;
use crate::parser::Parser;

pub struct CoalesceParselet;

impl Led for CoalesceParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::Op(Op::Coalesce));

        let rhs = parser.parse_expr(None)?;

        Ok(Coalesce::new(lhs, rhs).into())
    }

    fn get_precedence(&self) -> i8 {
        2
    }
}
