use Node;
use MutNode;
use NodeId;

///
/// A `Node` implementation for use in an `OptTree`.
///
/// `OptNode`s use `Option<NodeId>`s to reference all nearby `OptNode`s.
///
/// More information on the implications of this (vs. the way `VecNode`s are implemented) can be
/// found in the documentation for `VecTree` and `OptTree`.
///
pub struct OptNode<T> {
    data: T,
    parent: Option<NodeId>,
    prev_sibling: Option<NodeId>,
    next_sibling: Option<NodeId>,
    first_child: Option<NodeId>,
    last_child: Option<NodeId>,
}

impl<T> Node<T> for OptNode<T> {
    fn new(data: T) -> OptNode<T> {
        OptNode {
            data: data,
            parent: None,
            prev_sibling: None,
            next_sibling: None,
            first_child: None,
            last_child: None,
        }
    }

    fn data(&self) -> &T {
        &self.data
    }

    fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    fn replace_data(&mut self, mut data: T) -> T {
        ::std::mem::swap(&mut data, self.data_mut());
        data
    }

    fn parent(&self) -> Option<&NodeId> {
        self.parent.as_ref()
    }
}

impl<T> MutNode for OptNode<T> {
    fn set_parent(&mut self, parent: Option<NodeId>) {
        self.parent = parent;
    }
}

impl<T> OptNode<T> {
    ///
    /// Returns a `Some` value containing the `NodeId` of this `Node`'s previous sibling if it
    /// exists; returns `None` if it does not.
    ///
    /// **Note:** A `Node` cannot have a previous sibling until after it has been inserted into a
    /// `Tree`.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::OptNode;
    ///
    /// let five: OptNode<i32> = OptNode::new(5);
    ///
    /// assert!(five.prev_sibling().is_none());
    /// ```
    ///
    pub fn prev_sibling(&self) -> Option<&NodeId> {
        self.prev_sibling.as_ref()
    }

    ///
    /// Returns a `Some` value containing the `NodeId` of this `Node`'s next sibling if it
    /// exists; returns `None` if it does not.
    ///
    /// **Note:** A `Node` cannot have a next sibling until after it has been inserted into a
    /// `Tree`.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::OptNode;
    ///
    /// let five: OptNode<i32> = OptNode::new(5);
    ///
    /// assert!(five.next_sibling().is_none());
    /// ```
    ///
    pub fn next_sibling(&self) -> Option<&NodeId> {
        self.next_sibling.as_ref()
    }

    ///
    /// Returns a `Some` value containing the `NodeId` of this `Node`'s first child if it
    /// exists; returns `None` if it does not.
    ///
    /// **Note:** A `Node` cannot have a first child until after it has been inserted into a
    /// `Tree`.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::OptNode;
    ///
    /// let five: OptNode<i32> = OptNode::new(5);
    ///
    /// assert!(five.first_child().is_none());
    /// ```
    ///
    pub fn first_child(&self) -> Option<&NodeId> {
        self.first_child.as_ref()
    }

    ///
    /// Returns a `Some` value containing the `NodeId` of this `Node`'s last child if it
    /// exists; returns `None` if it does not.
    ///
    /// **Note:** A `Node` cannot have a last child until after it has been inserted into a
    /// `Tree`.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::OptNode;
    ///
    /// let five: OptNode<i32> = OptNode::new(5);
    ///
    /// assert!(five.last_child().is_none());
    /// ```
    ///
    pub fn last_child(&self) -> Option<&NodeId> {
        self.last_child.as_ref()
    }

    pub(crate) fn set_prev_sibling(&mut self, prev_sibling: Option<NodeId>) {
        self.prev_sibling = prev_sibling;
    }

    pub(crate) fn set_next_sibling(&mut self, next_sibling: Option<NodeId>) {
        self.next_sibling = next_sibling;
    }

    pub(crate) fn set_first_child(&mut self, first_child: Option<NodeId>) {
        self.first_child = first_child;
    }

    pub(crate) fn set_last_child(&mut self, last_child: Option<NodeId>) {
        self.last_child = last_child;
    }
}

#[cfg(test)]
mod opt_node_tests {
    use node::*;
    use NodeId;
    use snowflake::ProcessUniqueId;

    #[test]
    fn data() {
        let five = 5;
        let node = OptNode::new(five);
        assert_eq!(node.data(), &five);
    }

    #[test]
    fn data_mut() {
        let mut five = 5;
        let mut node = OptNode::new(five);
        assert_eq!(node.data_mut(), &mut five);
    }

    #[test]
    fn parent() {
        let mut node = OptNode::new(5);
        assert!(node.parent().is_none());

        let parent_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };

        node.set_parent(Some(parent_id.clone()));
        assert!(node.parent().is_some());

        assert_eq!(node.parent().unwrap().clone(), parent_id);
    }

    #[test]
    fn prev_sibling() {
        let mut node = OptNode::new(5);
        assert!(node.prev_sibling().is_none());

        let prev_sibling_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };

        node.set_prev_sibling(Some(prev_sibling_id.clone()));
        assert!(node.prev_sibling().is_some());

        assert_eq!(node.prev_sibling().unwrap().clone(), prev_sibling_id);
    }

    #[test]
    fn next_sibling() {
        let mut node = OptNode::new(5);
        assert!(node.next_sibling().is_none());

        let next_sibling_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };

        node.set_next_sibling(Some(next_sibling_id.clone()));
        assert!(node.next_sibling().is_some());

        assert_eq!(node.next_sibling().unwrap().clone(), next_sibling_id);
    }

    #[test]
    fn first_child() {
        let mut node = OptNode::new(5);
        assert!(node.first_child().is_none());

        let first_child_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };

        node.set_first_child(Some(first_child_id.clone()));
        assert!(node.first_child().is_some());

        assert_eq!(node.first_child().unwrap().clone(), first_child_id);
    }

    #[test]
    fn last_child() {
        let mut node = OptNode::new(5);
        assert!(node.last_child().is_none());

        let last_child_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };

        node.set_last_child(Some(last_child_id.clone()));
        assert!(node.last_child().is_some());

        assert_eq!(node.last_child().unwrap().clone(), last_child_id);
    }
}
