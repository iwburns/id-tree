mod vec_node;
mod opt_node;

use NodeId;
pub use self::vec_node::*;
pub use self::opt_node::*;

///
/// A container that wraps data in a given `Tree`.
///
pub trait Node<T> {
    ///
    /// Creates a new `Node` with the data provided.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let _one: VecNode<i32> = Node::new(1);
    /// ```
    ///
    fn new(data: T) -> Self
    where
        Self: Sized;

    ///
    /// Returns an immutable reference to the data contained within the `Node`.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let node_three: VecNode<i32> = Node::new(3);
    /// let three = 3;
    ///
    /// assert_eq!(node_three.data(), &three);
    /// ```
    ///
    fn data(&self) -> &T;

    ///
    /// Returns a mutable reference to the data contained within the `Node`.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let mut node_four: VecNode<i32> = Node::new(4);
    /// let mut four = 4;
    ///
    /// assert_eq!(node_four.data_mut(), &mut four);
    /// ```
    ///
    fn data_mut(&mut self) -> &mut T;

    ///
    /// Replaces this `Node`s data with the data provided.
    ///
    /// Returns the old value of data.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let mut node_four: VecNode<i32> = Node::new(3);
    ///
    /// // ops! lets correct this
    /// let three = node_four.replace_data(4);
    ///
    /// assert_eq!(node_four.data(), &4);
    /// assert_eq!(three, 3);
    /// ```
    ///
    fn replace_data(&mut self, data: T) -> T;

    ///
    /// Returns a `Some` value containing the `NodeId` of this `Node`'s parent if it exists; returns
    /// `None` if it does not.
    ///
    /// **Note:** A `Node` cannot have a parent until after it has been inserted into a `Tree`.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let five: VecNode<i32> = Node::new(5);
    ///
    /// assert!(five.parent().is_none());
    /// ```
    ///
    fn parent(&self) -> Option<&NodeId>;
}

pub(crate) trait MutNode {
    fn set_parent(&mut self, parent: Option<NodeId>);
}
