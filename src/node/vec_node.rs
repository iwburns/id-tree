use Node;
use MutNode;
use NodeId;

///
/// A `Node` builder that provides more control over how a `Node` is created.
///
pub struct VecNodeBuilder<T> {
    data: T,
    child_capacity: usize,
}

impl<T> VecNodeBuilder<T> {
    ///
    /// Creates a new `VecNodeBuilder` with the required data.
    ///
    /// ```
    /// use id_tree::VecNodeBuilder;
    ///
    /// let _node_builder = VecNodeBuilder::new(5);
    /// ```
    ///
    pub fn new(data: T) -> VecNodeBuilder<T> {
        VecNodeBuilder {
            data: data,
            child_capacity: 0,
        }
    }

    ///
    /// Set the child capacity of the `VecNodeBuilder`.
    ///
    /// As `Node`s are added to a `Tree`, parent and child references must be maintained. To do
    /// this, an allocation must be made every time a child is added to a `Node`.  Using this
    /// setting allows the `Node` to pre-allocate space for its children so that the allocations
    /// aren't made as children are added.
    ///
    /// _Use of this setting is recommended if you know the **maximum number** of children (not
    /// including grandchildren, great-grandchildren, etc.) that a `Node` will have **at any given
    /// time**_.
    ///
    /// ```
    /// use id_tree::VecNodeBuilder;
    ///
    /// let _node_builder = VecNodeBuilder::new(5).with_child_capacity(3);
    /// ```
    ///
    pub fn with_child_capacity(mut self, child_capacity: usize) -> VecNodeBuilder<T> {
        self.child_capacity = child_capacity;
        self
    }

    ///
    /// Build a `Node` based upon the current settings in the `VecNodeBuilder`.
    ///
    /// ```
    /// use id_tree::VecNodeBuilder;
    /// use id_tree::VecNode;
    /// use id_tree::Node;
    ///
    /// let node: VecNode<i32> = VecNodeBuilder::new(5)
    ///         .with_child_capacity(3)
    ///         .build();
    ///
    /// assert_eq!(node.data(), &5);
    /// assert_eq!(node.children().capacity(), 3);
    /// ```
    ///
    pub fn build(self) -> VecNode<T> {
        VecNode {
            data: self.data,
            parent: None,
            children: Vec::with_capacity(self.child_capacity),
        }
    }
}

///
/// A container that wraps data in a given `Tree`.
///
pub struct VecNode<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> Node<T> for VecNode<T> {
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
    fn new(data: T) -> VecNode<T> {
        VecNodeBuilder::new(data).build()
    }

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
    fn data(&self) -> &T {
        &self.data
    }

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
    fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

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
    fn replace_data(&mut self, mut data: T) -> T {
        ::std::mem::swap(&mut data, self.data_mut());
        data
    }

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
    /// Returns an immutable reference to a `Vec` containing the `NodeId`s of this `Node`'s
    /// children.
    ///
    /// **Note:** A `Node` cannot have any children until after it has been inserted into a `Tree`.
    ///
    /// ```
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let six: VecNode<i32> = Node::new(6);
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
mod vec_node_builder_tests {
    use node::*;

    #[test]
    fn test_new() {
        let five = 5;
        let node = VecNodeBuilder::new(5).build();
        assert_eq!(node.data(), &five);
        assert_eq!(node.children.capacity(), 0);
    }

    #[test]
    fn test_with_child_capacity() {
        let five = 5;
        let node = VecNodeBuilder::new(5).with_child_capacity(10).build();
        assert_eq!(node.data(), &five);
        assert_eq!(node.children.capacity(), 10);
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
