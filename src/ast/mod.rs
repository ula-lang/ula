use std::fmt;

use compilation::Compilable;

mod expr;
pub use self::expr::Expr;

mod stmt;
pub use self::stmt::Stmt;

mod node;
pub use self::node::Node;

pub mod decls;
pub mod exprs;
pub mod stmts;

pub struct Tree {
    nodes: Vec<Node>
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

    pub fn nodes(&self) -> impl Iterator<Item = &Node> {
        return self.nodes.iter()
    }
}

impl Compilable for Tree {
    fn compile(&self) -> String {
        let mut compiled = String::new();

        self.nodes().for_each(|node| {
            compiled.push_str(&node.compile());

            compiled.push_str("\r\n");
        });

        compiled
    }
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tree(Nodes({:?}))", self.nodes)
    }
}