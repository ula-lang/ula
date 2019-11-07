use std::fmt;

use crate::ast::Expr;
use crate::compilation::{Compilable, Scope};

#[derive(Clone)]
pub struct Table {
    len: usize,
    items: (Vec<Expr>, Vec<Expr>),
}

impl Table {
    pub fn new() -> Self {
        Self {
            len: 0,
            items: (Vec::new(), Vec::new()),
        }
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) where K: Into<Expr>, V: Into<Expr> {
        self.len += 1;
        self.items.0.push(key.into());
        self.items.1.push(value.into());
    }
}

impl Into<Expr> for Table {
    fn into(self) -> Expr {
        Expr::Table(self)
    }
}

impl Compilable for Table {
    fn compile(&self, scope: &Scope) -> String {
        let mut compiled = String::new();

        compiled.push('{');

        for i in 0..self.len {
            // Should be guaranteed as we have authority of `len`
            let (key, value) = (self.items.0.get(i).unwrap(), self.items.1.get(i).unwrap());

            compiled.push_str(&format!(r#"[{}] = {}"#, key.compile(scope), value.compile(scope)));

            if i + 1 < self.len {
                compiled.push_str(", ")
            }
        }

        compiled.push('}');

        compiled
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Table(Items({:?}))", self.items)
    }
}