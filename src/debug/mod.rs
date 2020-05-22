use ptree::TreeBuilder;

pub trait TreeNode {
    fn write_tree(&self, builder: &mut TreeBuilder);
}
