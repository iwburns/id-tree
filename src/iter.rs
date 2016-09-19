use super::tree::Tree;
use super::node::Node;
use super::NodeId;

//todo: see if we can do this without a stack.

pub struct PreOrderIterator<'a, T: 'a> {
    tree: &'a Tree<T>,
    starting_node_id: NodeId,
    node_stack: Vec<NodeId>,
}

impl<'a, T: 'a> Iterator for PreOrderIterator<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

