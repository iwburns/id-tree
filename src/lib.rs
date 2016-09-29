//! A library for creating and modifying Tree structures.
//!
//! In this implementation, the `Tree` owns all of the `Node`s and all inter-`Node` relationships are
//! managed with `NodeId`s.  This means that you will need a reference to the `Tree` to get a
//! reference to any `Node`'s parent or child `Node`s.
//!
//! It is also important to note that this library does not support arbitrary Graph creation.  Any
//! given Node can have **up to one parent** and **an arbitrary number of children**.
//!

extern crate snowflake;
use self::snowflake::ProcessUniqueId;

mod node;
mod tree;

pub use node::Node;
pub use tree::TreeBuilder;
pub use tree::Tree;

///
/// An identifier used to differentiate between Nodes within a Tree.
///
/// `NodeId`s are not something that you will ever have to worry about generating yourself.  `Tree`s
/// generate `NodeId`s as you insert `Node`s into them.
///
/// `NodeId`s are also specific to the Tree that generated them.  This means that if you have two `Tree`s
/// `A` and `B`, there's no worry of trying to access a Node in `A` with an identifier that came
/// from `B`.
///
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: usize,
}

trait MutableNode {
    fn set_parent(&mut self, parent: Option<NodeId>);
    fn add_child(&mut self, child: NodeId);
    fn children_mut(&mut self) -> &mut Vec<NodeId>;
}
