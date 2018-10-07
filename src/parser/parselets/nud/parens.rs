use ast::Expr;
use ast::exprs::Parens;
use lexer::Token;
use parser::parselets::Nud;
use parser::Parser;

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