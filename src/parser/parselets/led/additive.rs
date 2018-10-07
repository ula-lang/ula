use ast::Expr;
use lexer::{Op, Token};
use parser::parselets::Led;
use parser::Parser;
use ast::exprs::{Sub, Sum};

pub struct AdditiveParselet;

impl Led for AdditiveParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        let rhs = parser.parse_expr(Some(self.get_precedence()))?;

        match token {
            Token::Op(Op::Add) => {
                Ok(Sum::new(lhs, rhs).into())
            }

            Token::Op(Op::Sub) => {
                Ok(Sub::new(lhs, rhs).into())
            }

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        11
    }
}