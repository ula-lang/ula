use crate::ast::Expr;
use crate::ast::exprs::Cond;
use crate::lexer::{Op, Token};
use crate::parser::parselets::Led;
use crate::parser::Parser;

pub struct CondParselet;

impl Led for CondParselet {
    fn parse(&self, parser: &mut Parser, cond: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::Op(Op::Cond));

        let expr1 = parser.parse_expr(None)?;

        parser.expect(Token::Tilda)?;

        let expr2 = parser.parse_expr(None)?;

        Ok(Cond::new(cond, expr1, expr2).into())
    }

    fn get_precedence(&self) -> i8 {
        2
    }
}