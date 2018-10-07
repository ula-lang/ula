use ast::Expr;
use ast::exprs::Concat;
use lexer::{Op, Token};
use parser::parselets::Led;
use parser::Parser;

pub struct ConcatParselet;

impl Led for ConcatParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::Op(Op::Concat));

        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        Ok(Concat::new(lhs, rhs).into())
    }

    fn get_precedence(&self) -> i8 {
        11
    }
}