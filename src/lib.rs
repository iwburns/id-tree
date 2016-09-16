extern crate snowflake;
use self::snowflake::ProcessUniqueId;

mod node;
mod tree;

pub use node::Node;
pub use tree::TreeBuilder;
pub use tree::Tree;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: usize
}

trait MutableNode {
    fn set_parent(&mut self, parent: Option<NodeId>);
    fn add_child(&mut self, child: NodeId);
    fn children_mut(&mut self) -> &mut Vec<NodeId>;
}
