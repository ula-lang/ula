use ast::Expr;
use lexer::Token;
use parser::parselets::Led;
use parser::Parser;
use ast::exprs::{FCall, MCall, Ref};

pub struct CallParselet;

impl Led for CallParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        match token {
            Token::Colon => {
                let ident = parser.parse_ident()?;

                parser.expect(Token::LParens)?;

                let args = parser.parse_paren_args()?;

                Ok(MCall::new(lhs, FCall::new(Ref::new(ident), args)).into())
            }

            Token::LParens => {
                let args = parser.parse_paren_args()?;

                Ok(FCall::new(lhs, args).into())
            }

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        12
    }
}