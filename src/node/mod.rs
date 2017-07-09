mod vec_node;

use NodeId;
pub use self::vec_node::*;

pub trait Node<T> {
    fn new(data: T) -> Self where Self: Sized;
    fn data(&self) -> &T;
    fn data_mut(&mut self) -> &mut T;
    fn replace_data(&mut self, data: T) -> T;
    fn parent(&self) -> Option<&NodeId>;
}

pub(crate) trait MutNode {
    fn set_parent(&mut self, parent: Option<NodeId>);
    fn add_child(&mut self, child: NodeId);
    fn replace_child(&mut self, old: NodeId, new: NodeId);
    fn children_mut(&mut self) -> &mut Vec<NodeId>;
    fn set_children(&mut self, children: Vec<NodeId>);
    fn take_children(&mut self) -> Vec<NodeId>;
}
