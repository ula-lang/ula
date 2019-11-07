mod keyword;

pub use self::keyword::Keyword;

mod literal;

pub use self::literal::Literal;

mod op;

pub use self::op::Op;

mod token;

pub use self::token::Token;

pub struct Lexer;

impl Lexer {
    pub fn lex(mut input: &str) -> Result<Vec<Token>, String> {
        mod lexer_impl {
            use super::{
                Keyword,
                Literal,
                Op,
                Token,
            };
            use plex::lexer;

            lexer! {
                pub fn take_token(text: 'a) -> Result<Token, String>;
                // Whitespace
                r"[ \t\r\n]" => Ok(Token::Whitespace(text.chars().filter(|c| c == &'\n').count())),


                // Comment
                r"//[^\n]*" => Ok(Token::Comment(text.to_owned())),
                "/[*](~(.*[*]/.*))[*]/" => Ok(Token::Comment(text.to_owned())),


                // Generic
                r"\{" => Ok(Token::LBrace),
                r"\}" => Ok(Token::RBrace),
                r"\(" => Ok(Token::LParens),
                r"\)" => Ok(Token::RParens),
                r"\[" => Ok(Token::LBracket),
                r"\]" => Ok(Token::RBracket),
                ":"   => Ok(Token::Colon),
                ","   => Ok(Token::Comma),
                r"\~" => Ok(Token::Tilda),
                ";"   => Ok(Token::Semicolon),


                // Keyword
                "async"    => Ok(Keyword::Async.into()),
                "await"    => Ok(Keyword::Await.into()),
                "break"    => Ok(Keyword::Break.into()),
                "case"     => Ok(Keyword::Case.into()),
                "class"    => Ok(Keyword::Class.into()),
                "else"     => Ok(Keyword::Else.into()),
                "enum"     => Ok(Keyword::Enum.into()),
                "export"   => Ok(Keyword::Export.into()),
                "extends"  => Ok(Keyword::Extends.into()),
                "for"      => Ok(Keyword::For.into()),
                "foreach"  => Ok(Keyword::ForEach.into()),
                "from"     => Ok(Keyword::From.into()),
                "function" => Ok(Keyword::Function.into()),
                "if"       => Ok(Keyword::If.into()),
                "import"   => Ok(Keyword::Import.into()),
                "in"       => Ok(Keyword::In.into()),
                "var"      => Ok(Keyword::Var.into()),
                "local"    => Ok(Keyword::Local.into()),
                "new"      => Ok(Keyword::New.into()),
                "return"   => Ok(Keyword::Return.into()),
                "static"   => Ok(Keyword::Static.into()),
                "switch"   => Ok(Keyword::Switch.into()),
                "while"    => Ok(Keyword::While.into()),
                "yield"    => Ok(Keyword::Yield.into()),


                // Literals
                    // Boolean
                    "true"  => Ok(Literal::Bool(true).into()),
                    "false" => Ok(Literal::Bool(false).into()),

                    // Nil
                    "nil" => Ok(Literal::Nil.into()),

                    // Float
                    r"[0-9]+\.[0-9]+" => {
                        if let Ok(f) = text.parse::<f64>() {
                            Ok(Literal::Float(f).into())
                        } else {
                            Err(format!("Float {:?} is out of range", text))
                        }
                    }

                    // Integer
                    "[0-9]+" => {
                        if let Ok(i) = text.parse::<i64>() {
                            Ok(Literal::Integer(i).into())
                        } else {
                            Err(format!("Integer {:?} is out of range", text))
                        }
                    }

                    // String
                    r#""(\\.|[^"\\])*""# => Ok(Literal::String(text[1..text.len() - 1].to_owned()).into()),


                // Operators
                    // Assignment
                    "="     => Ok(Op::Assign.into()),
                    r"\+="  => Ok(Op::AddAssign.into()),
                    "&="    => Ok(Op::BAndAssign.into()),
                    r"\|="  => Ok(Op::BOrAssign.into()),
                    r"--"   => Ok(Op::Decr.into()),
                    "/="    => Ok(Op::DivAssign.into()),
                    r"\+\+" => Ok(Op::Incr.into()),
                    "<<="   => Ok(Op::LShiftAssign.into()),
                    "%="    => Ok(Op::ModAssign.into()),
                    r"\*="  => Ok(Op::MulAssign.into()),
                    ">>="   => Ok(Op::RShiftAssign.into()),
                    "-="    => Ok(Op::SubAssign.into()),
                    "^="    => Ok(Op::XorAssign.into()),

                    // Arithmetic
                    r"\+" => Ok(Op::Add.into()),
                    "-"   => Ok(Op::Sub.into()),
                    r"\*" => Ok(Op::Mul.into()),
                    "/"   => Ok(Op::Div.into()),
                    "%"   => Ok(Op::Mod.into()),

                    // Comparison
                    "==" => Ok(Op::Eq.into()),
                    "!=" => Ok(Op::Ne.into()),
                    ">"  => Ok(Op::Gt.into()),
                    "<"  => Ok(Op::Lt.into()),
                    ">=" => Ok(Op::GtEq.into()),
                    "<=" => Ok(Op::LtEq.into()),

                    // Logical
                    r"\&\&" => Ok(Op::LAnd.into()),
                    r"\|\|" => Ok(Op::LOr.into()),
                    "!"     => Ok(Op::Not.into()),

                    // Bitwise
                    r"\&"  => Ok(Op::BAnd.into()),
                    r"\|"  => Ok(Op::BOr.into()),
                    r"\^"  => Ok(Op::Xor.into()),
                    "<<"   => Ok(Op::LShift.into()),
                    ">>"   => Ok(Op::RShift.into()),

                    // Misc
                    r"\?\?"   => Ok(Op::Coalesce.into()),
                    r"\.\."   => Ok(Op::Concat.into()),
                    r"\?"     => Ok(Op::Cond.into()),
                    r"\."     => Ok(Op::Dot.into()),
                    r"\.\.\." => Ok(Op::Ellipsis.into()),
                    r"\$"     => Ok(Op::Interp.into()),
                    "#"       => Ok(Op::Len.into()),

                // Identifier
                "[a-zA-Z_][a-zA-Z0-9_]*" => Ok(Token::Ident(text.to_owned())),

                // Bad character
                "." => Err(format!("Unexpected character {:?} in input", text))
            }
        }

        let mut tokens = Vec::new();

        while let Some((token, remaining)) = lexer_impl::take_token(input) {
            input = remaining;

            tokens.push(token?);
        }

        Ok(tokens)
    }
}