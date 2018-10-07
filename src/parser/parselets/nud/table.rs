use ast::Expr;
use ast::exprs::{Const, Table};
use lexer::{Op, Token};
use parser::parselets::Nud;
use parser::Parser;

pub struct TableParselet;

impl Nud for TableParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expr, String> {
        assert_eq!(token, Token::LBrace);

        let (mut index, mut table) = (0, Table::new());

        while !parser.next_is(Token::RBrace) {
            let (key, value) = {
                match parser.peek(0)? {
                    // Expression key
                    Token::LBracket => {
                        let key = parser.parse_expr(None)?;

                        parser.expect(Token::RBracket)?;

                        parser.expect(Op::Assign)?;

                        let value = parser.parse_expr(None)?;

                        (key, value)
                    }

                    _ => {
                        // Don't accept assignment expressions
                        let expr = parser.parse_expr(Some(1))?;

                        if parser.next_is(Op::Assign) {
                            let key = {
                                match expr {
                                    Expr::Var(var) => Const::from(var.ident()).into(),

                                    t => return Err(
                                        format!(
                                            "{:?} cannot be used as a table key, perhaps you meant to surround it with brackets e.g `[<key>] = <value>`", t
                                        )
                                    )
                                }
                            };

                            parser.consume()?;

                            let value = parser.parse_expr(None)?;

                            (key, value)
                        } else {
                            index += 1;

                            (Const::from(index).into(), expr)
                        }
                    }
                }
            };

            if !parser.next_is(Token::RBrace) {
                parser.expect(Token::Comma)?;
            }

            table.insert(key, value);
        }

        parser.expect(Token::RBrace)?;

        Ok(table.into())
    }

    fn get_precedence(&self) -> i8 {
        15
    }
}