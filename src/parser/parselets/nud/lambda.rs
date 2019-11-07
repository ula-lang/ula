use crate::parser::parselets::Nud;
use crate::parser::Parser;
use crate::lexer::Token;
use crate::ast::Expr;
use crate::lexer::Keyword;
use crate::lexer::Op;
use crate::ast::exprs::Lambda;
use crate::ast::stmts::Return;

pub struct LambdaParselet;

impl Nud for LambdaParselet {
    fn parse(&self, parser: &mut Parser, mut token: Token) -> Result<Expr, String> {
        let flags = {
            if token == Token::Keyword(Keyword::Async) {
                token = parser.consume()?;

                (true,)
            } else {
                (false,)
            }
        };

        let params = {
            if token == Token::Op(Op::BOr) {
                let params = parser.parse_params()?;

                parser.expect(Op::BOr)?;

                params
            } else {
                assert_eq!(token, Token::Op(Op::LOr));

                Vec::new()
            }
        };

        let body = {
            if parser.next_is(Token::LBrace) {
                parser.parse_block()?
            } else {
                vec![Return::new(Some(parser.parse_expr(None)?)).into()]
            }
        };

        Ok(Lambda::new(flags, params, body).into())
    }

    fn get_precedence(&self) -> i8 {
        1
    }
}