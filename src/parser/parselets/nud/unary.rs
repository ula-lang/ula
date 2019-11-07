use crate::ast::Expr;
use crate::ast::exprs::{
    Assignment, Await, Concat,
    Const, FCall, Len, Neg,
    New, Not, Ref, Sub, Sum,
};
use crate::lexer::*;
use crate::parser::parselets::Nud;
use crate::parser::Parser;

pub struct UnaryParselet;

impl Nud for UnaryParselet {
    fn parse(&self, parser: &mut Parser, token: Token) -> Result<Expr, String> {
        match token {
            Token::Keyword(Keyword::Await) => {
                let rhs = parser.parse_expr(None)?;

                Ok(Await::new(rhs).into())
            }

            Token::Op(Op::Interp) => {
                match parser.consume()? {
                    Token::Literal(Literal::String(string)) => {
                        fn parse_expr_str(input: &str) -> Result<Expr, String> {
                            let in_tokens = Lexer::lex(input)?;

                            let mut parser = Parser::new(in_tokens);

                            return parser.parse_expr(None);
                        }

                        let (mut buf, mut exprs, mut level, mut escaped) = (String::new(), Vec::new(), 0usize, false);

                        for (i, c) in string.chars().into_iter().enumerate() {
                            if escaped {
                                buf.push(c);

                                escaped = false;

                                continue;
                            }

                            match c {
                                '{' => {
                                    if level > 0 {
                                        buf.push(c);
                                    } else {
                                        // Buf *might* contain some chars
                                        if buf.len() > 0 {
                                            exprs.push(Expr::Const(buf.into()));

                                            buf = String::new();
                                        }
                                    }

                                    level += 1;
                                }

                                '}' => {
                                    if level == 0 {
                                        return Err("Bad interpolation".to_owned());
                                    }

                                    level -= 1;

                                    if level > 0 {
                                        buf.push(c);
                                    } else {
                                        // Buf should now contain an expression!
                                        exprs.push(parse_expr_str(&buf)?);

                                        buf = String::new();
                                    }
                                }

                                '\\' => {
                                    if level == 0 {
                                        if let Some(next) = string.chars().nth(i + 1) {
                                            if next == '{' || next == '}' {
                                                escaped = true;

                                                continue;
                                            }
                                        }
                                    }

                                    buf.push(c);
                                }

                                c => buf.push(c)
                            }
                        }

                        if exprs.len() == 0 || buf.len() > 0 { }
                        exprs.push(Const::from(buf).into());

                        exprs.reverse();

                        let mut expr = exprs.pop().unwrap();

                        while let Some(next_expr) = exprs.pop() {
                            expr = Concat::new(expr, next_expr).into();
                        }

                        Ok(expr)
                    }

                    t => Err(format!("Unexpected {} after '$'", t))
                }
            }

            Token::Op(Op::Len) => {
                let rhs = parser.parse_expr(None)?;

                Ok(Len::new(rhs).into())
            }

            Token::Op(Op::Incr) => {
                let lhs = parser.parse_expr(None)?;

                Ok(Assignment::new(lhs.clone(), Sum::new(lhs, 1)).into())
            }

            Token::Op(Op::Decr) => {
                let lhs = parser.parse_expr(None)?;

                Ok(Assignment::new(lhs.clone(), Sub::new(lhs, 1)).into())
            }

            Token::Keyword(Keyword::New) => {
                let ident = parser.parse_ident()?;

                parser.expect(Token::LParens)?;

                let args = parser.parse_paren_args()?;

                Ok(New::new(ident.clone(), FCall::new(Ref::new(ident), args)).into())
            }

            Token::Op(Op::Sub) => {
                let rhs = parser.parse_expr(None)?;

                Ok(Neg::new(rhs).into())
            }

            Token::Op(Op::Not) => {
                let rhs = parser.parse_expr(None)?;

                Ok(Not::new(rhs).into())
            }

            _ => unreachable!()
        }
    }

    fn get_precedence(&self) -> i8 {
        14
    }
}