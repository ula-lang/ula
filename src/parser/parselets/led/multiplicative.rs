use ast::Expr;
use lexer::{Op, Token};
use parser::parselets::Led;
use parser::Parser;
use ast::exprs::{Div, Mod, Mul};

pub struct MultiplicativeParselet;

impl Led for MultiplicativeParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        match token {
            Token::Op(Op::Mul) => {
                Ok(Mul::new(lhs, rhs).into())
            }

            Token::Op(Op::Div) => {
                Ok(Div::new(lhs, rhs).into())
            }

            Token::Op(Op::Mod) => {
                Ok(Mod::new(lhs, rhs).into())
            }

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        13
    }
}