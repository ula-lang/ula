use crate::ast::Expr;
use crate::ast::exprs::Parens;
use crate::lexer::Token;
use crate::parser::parselets::Nud;
use crate::parser::Parser;

pub struct ParensParselet;

impl Nud for ParensParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::LParens);

        let expr = parser.parse_expr(None)?;

        parser.expect(Token::RParens)?;

        Ok(Parens::new(expr).into())
    }

    fn get_precedence(&self) -> i8 {
        15
    }
}