use Node;
use MutNode;
use NodeId;

///
/// A `Node` implementation for use in a `VecTree`.
///
/// `VecNode`s store their children in a `Vec<NodeId>` and only use an `Option<NodeId>` to
/// reference their parent.
///
/// More information on the implications of this (vs. the way `OptNode`s are implemented) can be
/// found in the documentation for `VecTree` and `OptTree`.
///
pub struct VecNode<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> Node<T> for VecNode<T> {
    fn new(data: T) -> VecNode<T> {
        VecNode {
            data: data,
            parent: None,
            children: Vec::new(),
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

impl<T> MutNode for VecNode<T> {
    fn set_parent(&mut self, parent: Option<NodeId>) {
        self.parent = parent;
    }
}

impl<T> VecNode<T> {
    ///
    /// Creates a new `Node` with the data provided and pre-allocates enough space for the given
    /// child_capacity.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let _one: VecNode<i32> = VecNode::new_with_child_capacity(1, 5);
    /// ```
    ///
    pub fn new_with_child_capacity(data: T, child_capacity: usize) -> VecNode<T> {
        VecNode {
            data: data,
            parent: None,
            children: Vec::with_capacity(child_capacity),
        }
    }

    ///
    /// Returns an immutable reference to a `Vec` containing the `NodeId`s of this `Node`'s
    /// children.
    ///
    /// **Note:** A `Node` cannot have any children until after it has been inserted into a `Tree`.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let six: VecNode<i32> = VecNode::new(6);
    ///
    /// assert_eq!(six.children().len(), 0);
    /// ```
    ///
    pub fn children(&self) -> &Vec<NodeId> {
        &self.children
    }

    pub(crate) fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    pub(crate) fn replace_child(&mut self, old: NodeId, new: NodeId) {
        let index = self.children()
            .iter()
            .enumerate()
            .find(|&(_, id)| id == &old)
            .unwrap()
            .0;

        let children = self.children_mut();
        children.push(new);
        children.swap_remove(index);
    }

    pub(crate) fn children_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.children
    }

    pub(crate) fn set_children(&mut self, children: Vec<NodeId>) {
        self.children = children;
    }

    pub(crate) fn take_children(&mut self) -> Vec<NodeId> {
        use std::mem;

        let mut empty = Vec::with_capacity(0);
        mem::swap(&mut self.children, &mut empty);
        empty //not so empty anymore
    }
}

#[cfg(test)]
mod vec_node_tests {
    use node::*;
    use NodeId;
    use snowflake::ProcessUniqueId;

    #[test]
    fn test_new() {
        let node = VecNode::new(5);
        assert_eq!(node.children.capacity(), 0);
    }

    #[test]
    fn test_new_with_child_capacity() {
        let node = VecNode::new_with_child_capacity(5, 6);
        assert_eq!(node.children.capacity(), 6);
    }

    #[test]
    fn test_data() {
        let five = 5;
        let node = VecNode::new(five);
        assert_eq!(node.data(), &five);
    }

    #[test]
    fn test_data_mut() {
        let mut five = 5;
        let mut node = VecNode::new(five);
        assert_eq!(node.data_mut(), &mut five);
    }

    #[test]
    fn test_parent() {
        let mut node = VecNode::new(5);
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
    fn test_children() {
        let mut node = VecNode::new(5);
        assert_eq!(node.children().len(), 0);

        let child_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };
        node.add_child(child_id.clone());

        assert_eq!(node.children().len(), 1);
        assert_eq!(node.children().get(0).unwrap(), &child_id);

        let mut node = VecNode::new(5);
        assert_eq!(node.children().len(), 0);

        let child_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };
        node.children_mut().push(child_id.clone());

        assert_eq!(node.children().len(), 1);
        assert_eq!(node.children().get(0).unwrap(), &child_id);
    }
}
