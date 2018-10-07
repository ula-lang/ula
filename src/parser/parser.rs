use ast::{Expr, Node, Stmt, Tree};
use ast::decls::*;
use ast::stmts::*;
use lexer::*;
use parser::parselets::{Led, Nud};
use parser::parselets::led::*;
use parser::parselets::nud::*;
use std::collections::VecDeque;

pub struct Parser {
    line: usize,
    tokens: VecDeque<(usize, Token)>,
}

impl Parser {
    pub fn new(in_tokens: Vec<Token>) -> Parser {
        let tokens = {
            let mut line = 1;

            in_tokens
                .into_iter()
                .filter_map(|t| match t {
                    Token::Whitespace(lines) => {
                        line += lines;

                        None
                    }

                    Token::Comment(text) => {
                        line += text.chars().filter(|c| c == &'\n').count();

                        None
                    }

                    token => Some((line, token)),
                })
                .collect()
        };


        Self {
            line: 1,
            tokens,
        }
    }

    pub fn parse(&mut self) -> Result<Tree, Vec<String>> {
        let mut tree = Tree::new();
        let mut errors = Vec::new();

        while self.tokens.len() > 0 {
            match self.parse_node() {
                Ok(node) => {
                    tree.add_node(node);
                }

                Err(error) => {
                    errors.push(format!("{} on line {}", error, self.line));


                    // Attempt error recovery
                    while !self.next_is(Token::RBrace) {
                        if let Err(_) = self.consume() {
                            break;
                        }
                    }

                    if self.next_is(Token::RBrace) {
                        self.consume().unwrap();
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(tree)
        } else {
            Err(errors)
        }
    }

    fn parse_node(&mut self) -> Result<Node, String> {
        let flags = self.parse_flags()?;

        match self.peek(0)? {
            Token::Keyword(Keyword::Class) => {
                if flags.2 {
                    return Err(format!("The 'async' flag is not valid for type 'class'"));
                }

                Ok(self.parse_class_decl((flags.0, flags.1))?.into())
            }

            Token::Keyword(Keyword::Enum) => {
                if flags.1 {
                    return Err(format!("The 'static' flag is not valid for type 'enum'"));
                }

                if flags.2 {
                    return Err(format!("The 'async' flag is not valid for type 'enum"));
                }

                Ok(self.parse_enum_decl((flags.0, ))?.into())
            }

            Token::Keyword(Keyword::Function) => {
                if flags.1 {
                    return Err(format!("The 'static' flag is not valid for function declarations"));
                }

                Ok(self.parse_func_decl((flags.0, true, flags.2))?.into())
            }

            _ => Ok(self.parse_stmt()?.into()),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        Ok(match self.peek(0)? {
            Token::Keyword(Keyword::Export) => self.parse_export_stmt()?.into(),

            Token::Keyword(Keyword::For) => self.parse_for_stmt()?.into(),

            Token::Keyword(Keyword::If) => self.parse_if_else_stmt()?.into(),

            Token::Keyword(Keyword::Import) => self.parse_import_stmt()?.into(),

            Token::Keyword(Keyword::Return) => self.parse_return_stmt()?.into(),

            Token::Keyword(Keyword::Var) => self.parse_var_decl_stmt()?.into(),

            Token::Keyword(Keyword::While) => self.parse_while_stmt()?.into(),

            Token::Keyword(Keyword::Yield) => self.parse_yield_stmt()?.into(),

            _ => self.parse_expr_stmt()?.into(),
        })
    }

    pub fn parse_expr(&mut self, precedence: Option<i8>) -> Result<Expr, String> {
        let precedence = precedence.unwrap_or(0);

        let token = self.consume()?;

        let nud_parselet = Self::get_nud_parselet(&token)?;

        let mut expr = nud_parselet.parse(self, token)?;

        while self.tokens.len() > 0 {
            if let Some(led_parselet) = Self::get_led_parselet(&self.peek(0)?)? {
                if precedence > led_parselet.get_precedence() {
                    // Done parsing, precedence level of the next token is too high
                    break;
                }

                let token = self.consume()?;

                expr = led_parselet.parse(self, expr, token)?
            } else {
                // Done parsing, no parselet for current token
                break;
            }
        }

        Ok(expr)
    }

    // <Declaration Parsers>
    fn parse_class_decl(&mut self, flags: (bool, bool)) -> Result<ClassDecl, String> {
        self.expect(Keyword::Class)?;

        let ident = self.parse_ident()?;

        let mut parent = None;

        if self.next_is(Keyword::Extends) {
            self.consume()?;

            parent = Some(self.parse_ident()?)
        }

        self.expect(Token::LBrace)?;

        let (mut ctor, mut fields, mut methods) = (None, Vec::new(), Vec::<FuncDecl>::new());

        while !self.next_is(Token::RBrace) {
            let m_flags = self.parse_flags()?;

            match self.peek(0)? {
                Token::Ident(ref ident) => {
                    self.consume()?;

                    if m_flags == (false, true, false) {
                        let mut field = (ident.to_owned(), None);

                        if self.next_is(Op::Assign) {
                            self.consume()?;

                            field.1 = Some(self.parse_expr(None)?);
                        }

                        self.expect(Token::Semicolon)?;

                        fields.push(field);
                    } else {
                        return Err("Unexpected identifier in class declaration".to_owned());
                    }
                }

                Token::Keyword(Keyword::Function) => {
                    let method = self.parse_func_decl(m_flags)?;

                    if methods.iter().any(|m| m.ident == method.ident) {
                        return Err(format!(
                            "Duplicate method declaration {:?} for class {:?}", method.ident, ident
                        ));
                    }

                    if !flags.1 && method.ident == ident {
                        if ctor.is_some() {
                            return Err(format!("Duplicate constructor definition for class {:?}", ident));
                        }

                        if method.is_static() {
                            return Err(format!("Constructor for {:?} is marked as 'static'", ident));
                        }

                        ctor = Some(method);
                    } else {
                        methods.push(method);
                    }
                }

                t => return Err(format!("Unexpected {:?} in class declaration", t))
            }
        }

        self.expect(Token::RBrace)?;

        // If the class is not staticm require a constructor ro be provided
        if !flags.1 && ctor.is_none() {
            return Err(format!("No constructor found for non-static class {:?}", ident));
        }

        Ok(ClassDecl::new(flags, ident, parent, ctor, fields, methods))
    }

    fn parse_enum_decl(&mut self, flags: (bool, )) -> Result<EnumDecl, String> {
        self.expect(Keyword::Enum)?;

        let ident = self.parse_ident()?;

        self.expect(Token::LBrace)?;

        let fields = self.parse_delimited(Token::Comma, |p| {
            let mut field = (p.parse_ident()?, None);

            if p.next_is(Op::Assign) {
                p.consume()?;

                field.1 = Some(p.parse_expr(None)?);
            }

            Ok(field)
        }, |t| t == Token::RBrace)?;

        self.expect(Token::RBrace)?;

        Ok(EnumDecl::new(flags, ident, fields))
    }

    fn parse_func_decl(&mut self, flags: (bool, bool, bool)) -> Result<FuncDecl, String> {
        self.expect(Keyword::Function)?;

        let ident = self.parse_ident()?;

        self.expect(Token::LParens)?;

        let params = self.parse_params()?;

        self.expect(Token::RParens)?;

        let body = self.parse_block()?;

        Ok(FuncDecl::new(flags, ident, params, body))
    }
    // </Declaration Parsers>

    // <Statement Parsers>
    fn parse_export_stmt(&mut self) -> Result<Export, String> {
        self.expect(Keyword::Export)?;

        let idents = self.parse_delimited(Token::Comma, |p| {
            p.parse_ident()
        }, |t| t == Token::Semicolon)?;

        if idents.len() < 1 {
            return Err("Cannot export 0 items".to_owned());
        }

        self.expect(Token::Semicolon)?;

        Ok(Export::new(idents))
    }

    fn parse_expr_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr(None)?;

        self.expect(Token::Semicolon)?;

        Ok(Stmt::Expr(expr))
    }

    fn parse_for_stmt(&mut self) -> Result<For, String> {
        self.expect(Keyword::For)?;

        self.expect(Token::LParens)?;

        let init = self.parse_var_decl_stmt()?;

        let cond = self.parse_expr(None)?;

        self.expect(Token::Semicolon)?;

        let iter = self.parse_expr(None)?;

        self.expect(Token::RParens)?;

        let body = self.parse_block()?;

        Ok(For::new(init, cond, iter, body))
    }

    fn parse_if_else_stmt(&mut self) -> Result<IfElse, String> {
        self.expect(Keyword::If)?;

        let cond = self.parse_expr(None)?;

        let (body, mut else_body) = (self.parse_block()?, None);

        if self.next_is(Keyword::Else) {
            self.consume()?;

            if self.next_is(Keyword::If) {
                else_body = Some(vec![self.parse_if_else_stmt()?.into()])
            } else {
                else_body = Some(self.parse_block()?)
            }
        }

        Ok(IfElse::new(cond, body, else_body))
    }

    fn parse_import_stmt(&mut self) -> Result<Import, String> {
        self.expect(Keyword::Import)?;

        let idents = self.parse_delimited(Token::Comma, |p| {
            p.parse_ident()
        }, |t| t == Token::Keyword(Keyword::From))?;

        if idents.len() < 1 {
            return Err("Cannot import 0 items".to_owned());
        }

        self.expect(Keyword::From)?;

        let source_file = {
            match self.consume()? {
                Token::Literal(Literal::String(source_file)) => source_file,

                t => return Err(format!("Expecting string, got {:?}", t))
            }
        };

        self.expect(Token::Semicolon)?;

        Ok(Import::new(idents, source_file))
    }

    fn parse_return_stmt(&mut self) -> Result<Return, String> {
        self.expect(Keyword::Return)?;

        let expr = {
            if self.next_is(Token::Semicolon) {
                None
            } else {
                Some(self.parse_expr(None)?)
            }
        };

        self.expect(Token::Semicolon)?;

        Ok(Return::new(expr))
    }

    fn parse_var_decl_stmt(&mut self) -> Result<VarDecl, String> {
        self.expect(Keyword::Var)?;

        let idents = self.parse_delimited(
            Token::Comma,
            |p| p.parse_ident(),
            |t| t == Op::Assign.into() || t == Token::Semicolon,
        )?;

        let mut init_exprs = None;

        if !self.next_is(Token::Semicolon) {
            self.expect(Op::Assign)?;

            init_exprs = Some(
                self.parse_delimited(
                    Token::Comma,
                    |p| p.parse_expr(None),
                    |t| t == Token::Semicolon,
                )?
            );
        }

        self.expect(Token::Semicolon)?;

        Ok(VarDecl::new(idents, init_exprs))
    }

    fn parse_while_stmt(&mut self) -> Result<While, String> {
        self.expect(Keyword::While)?;

        Ok(While::new(self.parse_expr(None)?, self.parse_block()?))
    }

    fn parse_yield_stmt(&mut self) -> Result<Yield, String> {
        self.expect(Keyword::Yield)?;

        let expr = {
            if self.next_is(Token::Semicolon) {
                None
            } else {
                Some(self.parse_expr(None)?)
            }
        };

        self.expect(Token::Semicolon)?;

        Ok(Yield::new(expr))
    }
    // </Statement Parsers>

    // <Helpers>
    pub fn consume(&mut self) -> Result<Token, String> {
        match self.tokens.pop_front() {
            Some((new_line, token)) => {
                self.line = new_line;

                Ok(token)
            }

            None => Err("Unexpected EOF".to_owned())
        }
    }

    pub fn expect<E>(&mut self, expected: E) -> Result<(), String> where E: Into<Token> {
        let (expected, got) = (expected.into(), self.consume()?);

        if got == expected {
            Ok(())
        } else {
            Err(format!("Unexpected {}, expecting {}", got, expected))
        }
    }

    pub fn next_is<E>(&mut self, expected: E) -> bool where E: Into<Token> {
        match self.peek(0) {
            Ok(got) => {
                got == expected.into()
            }

            Err(_) => false
        }
    }

    pub fn next_is_in<P>(&mut self, possibilities: &[P]) -> bool where P: Into<Token> + Clone {
        for possibility in possibilities {
            if self.next_is(possibility.clone().into()) {
                return true;
            }
        }

        false
    }
    // </Helpers>

    // <Parse Helpers>
    pub fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(Token::LBrace)?;

        let mut stmts = Vec::new();

        while !self.next_is(Token::RBrace) {
            stmts.push(self.parse_stmt()?);
        }

        self.consume()?;

        Ok(stmts)
    }

    pub fn parse_delimited<T, D, P, IE>(&mut self, delim: D, parse: P, is_end: IE) -> Result<Vec<T>, String>
        where
            D: Into<Token>,
            P: Fn(&mut Parser) -> Result<T, String>,
            IE: Fn(Token) -> bool
    {
        let (mut items, delim) = (Vec::new(), delim.into());

        while !is_end(self.peek(0)?) {
            items.push(parse(self)?);

            if !is_end(self.peek(0)?) {
                self.expect(delim.clone())?;
            }
        }

        Ok(items)
    }

    /// Returns flags tuple (0: local, 1: static, 2: async)
    fn parse_flags(&mut self) -> Result<(bool, bool, bool), String> {
        let mut flags = (false, false, false);

        while self.next_is_in(&[Keyword::Local, Keyword::Static, Keyword::Async]) {
            match self.consume()? {
                Token::Keyword(Keyword::Local) => {
                    if flags.0 {
                        return Err(format!("Duplicate flag 'local'"));
                    }

                    flags.0 = true;
                }

                Token::Keyword(Keyword::Static) => {
                    if flags.1 {
                        return Err(format!("Duplicate flag 'static'"));
                    }

                    flags.1 = true;
                }

                Token::Keyword(Keyword::Async) => {
                    if flags.2 {
                        return Err(format!("Duplicate flag 'async'"));
                    }

                    flags.2 = true;
                }

                _ => unreachable!()
            }
        }

        Ok(flags)
    }

    pub fn parse_ident(&mut self) -> Result<String, String> {
        match self.consume()? {
            Token::Ident(ident) => Ok(ident),

            t => Err(format!("Unexpected {}, expecting identifier", t))
        }
    }

    pub fn parse_params(&mut self) -> Result<Vec<(String, Option<Expr>)>, String> {
        self.parse_delimited(Token::Comma, |p| {
            let ident = {
                if p.next_is(Op::Ellipsis) {
                    p.consume()?;

                    "...".to_owned()
                } else {
                    p.parse_ident()?
                }
            };

            let mut param = (ident, None);

            if p.next_is(Op::Assign) {
                p.consume()?;

                param.1 = Some(p.parse_expr(None)?);
            }

            Ok(param)
        }, |t| t == Token::RBrace || t == Token::RParens || t == Token::Op(Op::BOr))
    }

    pub fn parse_paren_args(&mut self) -> Result<Vec<Expr>, String> {
        let args = self.parse_delimited(Token::Comma, |p| {
            p.parse_expr(None)
        }, |t| t == Token::RParens)?;

        self.expect(Token::RParens)?;

        Ok(args)
    }

    pub fn peek(&mut self, n: usize) -> Result<Token, String> {
        match self.tokens.iter().nth(n) {
            Some((new_line, token)) => {
                self.line = *new_line;

                Ok(token.clone())
            }

            None => Err("Unexpected EOF".to_owned())
        }
    }
    // </Parse Helpers>


    // <Parselet Fetchers>
    fn get_led_parselet(token: &Token) -> Result<Option<Box<Led>>, String> {
        match token {
            // Additive
            &Token::Op(Op::Add)
            | &Token::Op(Op::Sub) => Ok(Some(Box::new(AdditiveParselet))),

            // Assignment
            &Token::Op(Op::Assign)
            | &Token::Op(Op::AddAssign)
            | &Token::Op(Op::SubAssign)
            | &Token::Op(Op::MulAssign)
            | &Token::Op(Op::DivAssign)
            | &Token::Op(Op::ModAssign)
            | &Token::Op(Op::BAndAssign)
            | &Token::Op(Op::BOrAssign)
            | &Token::Op(Op::XorAssign)
            | &Token::Op(Op::LShiftAssign)
            | &Token::Op(Op::RShiftAssign) => Ok(Some(Box::new(AssignmentParselet))),

            // Bitwise AND
            Token::Op(Op::BAnd) => Ok(Some(Box::new(BAndParselet))),

            // Bitwise OR
            Token::Op(Op::BOr) => Ok(Some(Box::new(BOrParselet))),

            // Call
            &Token::LParens
            | &Token::Colon => Ok(Some(Box::new(CallParselet))),

            // Concat
            &Token::Op(Op::Concat) => Ok(Some(Box::new(ConcatParselet))),

            // Cond
            &Token::Op(Op::Cond) => Ok(Some(Box::new(CondParselet))),

            // Dot
            &Token::Op(Op::Dot) => Ok(Some(Box::new(DotParselet))),

            // Equality
            &Token::Op(Op::Eq)
            | &Token::Op(Op::Ne) => Ok(Some(Box::new(EqualityParselet))),

            // Index
            &Token::LBracket => Ok(Some(Box::new(IndexParselet))),

            // Multiplicative
            &Token::Op(Op::Mul)
            | &Token::Op(Op::Div)
            | &Token::Op(Op::Mod) => Ok(Some(Box::new(MultiplicativeParselet))),

            // Postfix
            &Token::Op(Op::Incr)
            | &Token::Op(Op::Decr) => Ok(Some(Box::new(PostfixParselet))),

            // Relational
            &Token::Op(Op::Lt)
            | &Token::Op(Op::Gt)
            | &Token::Op(Op::LtEq)
            | &Token::Op(Op::GtEq) => Ok(Some(Box::new(RelationalParselet))),

            // Shift
            &Token::Op(Op::LShift)
            | Token::Op(Op::RShift) => Ok(Some(Box::new(ShiftParselet))),

            // Logical AND
            &Token::Op(Op::LAnd) => Ok(Some(Box::new(LAndParselet))),

            // Logical OR
            &Token::Op(Op::LOr) => Ok(Some(Box::new(LOrParselet))),

            // XOR
            &Token::Op(Op::Xor) => Ok(Some(Box::new(XorParselet))),

            _ => Ok(None)
        }
    }

    fn get_nud_parselet(token: &Token) -> Result<Box<Nud>, String> {
        match token {
            // Const
            &Token::Literal(_) => Ok(Box::new(ConstParselet)),

            // Lambda
            &Token::Op(Op::LOr)
            | &Token::Op(Op::BOr)
            | &Token::Keyword(Keyword::Async) => Ok(Box::new(LambdaParselet)),

            // Parens
            &Token::LParens => Ok(Box::new(ParensParselet)),

            // Reference
            &Token::Ident(_)
            | &Token::Op(Op::Ellipsis) => Ok(Box::new(RefParselet)),

            // Table
            &Token::LBrace => Ok(Box::new(TableParselet)),

            // Unary
            &Token::Keyword(Keyword::Await)
            | &Token::Keyword(Keyword::New)
            | &Token::Op(Op::Interp)
            | &Token::Op(Op::Len)
            | &Token::Op(Op::Incr)
            | &Token::Op(Op::Decr)
            | &Token::Op(Op::Sub)
            | &Token::Op(Op::Not) => Ok(Box::new(UnaryParselet)),

            t => Err(format!("Unexpected {}, expecting expression", t))
        }
    }
    // </Parselet Fetchers>
}
