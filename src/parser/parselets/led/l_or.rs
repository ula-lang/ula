use ast::Expr;
use ast::exprs::LOr;
use lexer::{Op, Token};
use parser::parselets::Led;
use parser::Parser;

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