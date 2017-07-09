mod vec_node;
mod opt_node;

use NodeId;
pub use self::vec_node::*;
pub use self::opt_node::*;

//todo: add documentation here and remove from opt_node + vec_node

pub trait Node<T> {
    fn new(data: T) -> Self where Self: Sized;
    fn data(&self) -> &T;
    fn data_mut(&mut self) -> &mut T;
    fn replace_data(&mut self, data: T) -> T;
    fn parent(&self) -> Option<&NodeId>;
}

pub(crate) trait MutNode {
    fn set_parent(&mut self, parent: Option<NodeId>);
}
