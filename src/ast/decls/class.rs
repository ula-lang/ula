use std::fmt;

use crate::ast::decls::FuncDecl;
use crate::ast::{Expr, Node, Stmt};
use crate::ast::stmts::{IfElse, Return, VarDecl};
use crate::ast::exprs::{Assignment, Const, Eq, FCall, Table, Ref};
use crate::compilation::{Compilable, Scope};

pub struct ClassDecl {
    /// 0: Local, 1: Static
    flags: (bool, bool),
    ident: String,
    parent: Option<String>,
    ctor: Option<FuncDecl>,
    fields: Vec<(String, Option<Expr>)>,
    methods: Vec<FuncDecl>,
}

impl ClassDecl {
    pub fn new(
        flags: (bool, bool),
        ident: String,
        parent: Option<String>,
        mut ctor: Option<FuncDecl>,
        fields: Vec<(String, Option<Expr>)>,
        mut methods: Vec<FuncDecl>,
    ) -> Self {
        if flags.1 && ctor.is_some() {
            unreachable!()
        } else if !flags.1 && ctor.is_none() {
            unreachable!()
        }

        for method in methods.iter_mut() {
            let sep = {
                if method.is_static() {
                    '.'
                } else {
                    ':'
                }
            };

            method.ident = format!("{}{}{}", ident, sep, method.ident);

            method.body_mut().insert(
                0,
                VarDecl::new(
                    vec![
                        "Self".to_owned()
                    ],
                    vec![
                        Ref::new(ident.clone()).into()
                    ].into()
                ).into()
            );
        }

        if let Some(ref mut ctor) = ctor {
            ctor.ident = format!("{}:{}", ident, ident);

            ctor.body_mut().insert(0, {
                IfElse::new(Eq::new(Ref::new("self"), Const::Nil), vec![
                    Stmt::Expr(Assignment::new(Ref::new("self"), Table::new()).into()),
                    Stmt::Expr(FCall::new(Ref::new("setmetatable"), vec![
                        Ref::new("self").into(),
                        Ref::new(ident.clone()).into()
                    ]).into())
                ], None).into()
            });

            if let Some(ref parent) = parent {
                ctor.body_mut().insert(1, {
                    FuncDecl::new((true, true, false), "super", vec![("...".to_owned(), None)], vec![
                        Stmt::Expr(FCall::new(Ref::new(format!("self.{}", parent)), vec![
                            Ref::new("self").into(),
                            Ref::new("...").into()
                        ]).into())
                    ]).into()
                });
            }

            ctor.body_mut().push(Return::new(Some(Ref::new("self"))).into());
        }

        Self {
            flags,
            ident,
            parent,
            ctor,
            fields,
            methods,
        }
    }

    #[inline(always)]
    pub fn is_local(&self) -> bool {
        self.flags.0
    }

    #[inline(always)]
    pub fn is_static(&self) -> bool {
        self.flags.1
    }
}

impl Compilable for ClassDecl {
    fn compile(&self, scope: &Scope) -> String {
        scope.add_class(&self.ident);

        let mut compiled = String::new();

        compiled.push_str(&format!("--[[ Begin class: {:?} ]]--\r\n", self.ident));

        if self.is_local() {
            compiled.push_str("local ");
        }

        compiled.push_str(&format!("{} = {{", self.ident));

        for (i, &(ref ident, ref def)) in self.fields.iter().enumerate() {
            match def {
                &Some(ref def) => {
                    compiled.push_str(&format!("\r\n    {} = {}", ident, def.compile(scope)));
                }

                &None => {
                    compiled.push_str(&format!("\r\n    {} = nil", ident));
                }
            }

            if i + 1 < self.fields.len() {
                compiled.push(',');
            }

            compiled.push_str("\r\n");
        }

        compiled.push_str(&format!(
            "}}\r\n{}.__index = {};\r\n",
            self.ident, self.ident
        ));

        if let Some(ref parent) = self.parent {
            compiled.push_str(&format!("setmetatable({}, {});", self.ident, parent));
        }

        if let Some(ref ctor) = self.ctor {
            // Add ctor
            compiled.push_str("\r\n--[[ <Constructor> ]]--\r\n");
            compiled.push_str(&ctor.compile(scope));
            compiled.push_str("\r\n--[[ </Constructor> ]]--\r\n");
        }

        // Add methods
        if self.methods.len() > 0 {
            compiled.push_str("\r\n--[[ <Methods> ]]--\r\n");
            for (i, method) in self.methods.iter().enumerate() {
                compiled.push_str(&method.compile(scope));

                if i + 1 < self.methods.len() {
                    compiled.push_str("\r\n\r\n");
                }
            }
            compiled.push_str("\r\n--[[ </Methods> ]]--\r\n");
        }

        compiled.push_str(&format!("--[[ End class: {:?} ]]--\r\n", self.ident));

        compiled
    }
}

impl Into<Node> for ClassDecl {
    fn into(self) -> Node {
        Node::ClassDecl(self)
    }
}

impl fmt::Debug for ClassDecl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ClassDecl(")?;

        for (i, &(ref ident, ref def)) in self.fields.iter().enumerate() {
            if i == 0 {
                write!(f, "Fields(")?;
            }

            write!(f, "{}", ident)?;

            match def {
                &Some(ref expr) => {
                    write!(f, " = {:?}", expr)?;
                }

                &None => (),
            }

            if i + 1 < self.fields.len() {
                write!(f, ", ")?;
            } else if i + 1 == self.fields.len() {
                write!(f, "), ")?;
            }
        }

        write!(f, "Ctor({:?})", self.ctor)?;

        for (i, method) in self.methods.iter().enumerate() {
            if i == 0 {
                write!(f, ", Methods(")?;
            }

            write!(f, "{:?}", method)?;

            if i + 1 < self.methods.len() {
                write!(f, ", ")?;
            } else if i + 1 == self.methods.len() - 1 {
                write!(f, ")")?;
            }
        }

        write!(f, ")")
    }
}
