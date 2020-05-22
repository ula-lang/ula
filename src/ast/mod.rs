use std::{fmt, io};
use std::borrow::Cow;
use std::io::{Cursor, Error};

use ptree::{print_tree, Style, TreeBuilder, TreeItem};

use crate::compilation::{Compilable, Scope};
use crate::debug::TreeNode;

pub use self::expr::Expr;
pub use self::node::Node;
pub use self::stmt::Stmt;

mod expr;
mod stmt;
mod node;
pub mod decls;
pub mod exprs;
pub mod stmts;
pub mod util;

#[derive(Clone, Debug)]
pub struct Tree {
    nodes: Vec<Node>
}

#[derive(Clone, Debug)]
struct Flags {
    pub local: bool,
    pub static_: bool,
    pub async_: bool,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            nodes: Vec::new()
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node)
    }

    pub fn nodes(&self) -> impl Iterator<Item=&Node> {
        self.nodes.iter()
    }

    pub fn pretty_print(&self) -> io::Result<()> {
        let mut builder = TreeBuilder::new("Tree".to_owned());

        for node in &self.nodes {
            node.write_tree(&mut builder);
        }

        print_tree(&builder.build())
    }
}

impl Compilable for Tree {
    fn compile(&self, scope: &Scope) -> String {
        let mut compiled = String::new();

        self.nodes().for_each(|node| {
            compiled.push_str(&node.compile(scope));

            compiled.push_str("\r\n");
        });

        compiled
    }
}

impl From<Vec<Node>> for Tree {
    fn from(nodes: Vec<Node>) -> Self {
        Self {
            nodes
        }
    }
}
