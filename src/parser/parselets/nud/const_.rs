use crate::ast::Expr;
use crate::ast::exprs::Const;
use crate::lexer::Token;
use crate::parser::parselets::Nud;
use crate::parser::Parser;

pub struct ConstParselet;

impl Nud for ConstParselet {
    fn parse(&self, _parser: &mut Parser, token: Token) -> Result<Expr, String> {
        if let Token::Literal(literal) = token {
            let const_expr: Const = literal.into();

            Ok(const_expr.into())
        } else {
            unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        15
    }
}