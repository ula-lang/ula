use ast::Expr;
use ast::exprs::Index;
use lexer::Token;
use parser::parselets::Led;
use parser::Parser;

pub struct IndexParselet;

impl Led for IndexParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::LBracket);

        let index = parser.parse_expr(None)?;

        parser.expect(Token::RBracket)?;

        Ok(Index::new(lhs, index).into())
    }

    fn get_precedence(&self) -> i8 {
        15
    }
}