use super::NodeId;
use super::MutableNode;

///
/// A container that wraps data in a given Tree.
///
pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> Node<T> {
    ///
    /// Creates a new Node with the data provided.
    ///
    /// ```
    /// use id_tree::Node;
    ///
    /// let _one: Node<i32> = Node::new(1);
    /// ```
    ///
    pub fn new(data: T) -> Node<T> {
        Node::new_with_child_capacity(data, 0)
    }

    ///
    /// Creates a new Node with the data provided and pre-allocates space for the given number of
    /// children.
    ///
    /// ```
    /// use id_tree::Node;
    ///
    /// let _two: Node<i32> = Node::new_with_child_capacity(2, 3); //will have pre-allocated space for 3 children
    /// ```
    ///
    //todo: make a NodeBuilder for this kind of thing
    pub fn new_with_child_capacity(data: T, capacity: usize) -> Node<T> {
        Node {
            data: data,
            parent: None,
            children: Vec::with_capacity(capacity),
        }
    }

    ///
    /// Returns an immutable reference to the data contained within the Node.
    ///
    /// ```
    /// use id_tree::Node;
    ///
    /// let node_three: Node<i32> = Node::new(3);
    /// let three = 3;
    ///
    /// assert_eq!(node_three.data(), &three);
    /// ```
    ///
    pub fn data(&self) -> &T {
        &self.data
    }

    ///
    /// Returns a mutable reference to the data contained within the Node.
    ///
    /// ```
    /// use id_tree::Node;
    ///
    /// let mut node_four: Node<i32> = Node::new(4);
    /// let mut four = 4;
    ///
    /// assert_eq!(node_four.data_mut(), &mut four);
    /// ```
    ///
    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    ///
    /// Returns a `Some` value containing the NodeId of this Node's parent if it exists; returns `None` if it does not.
    ///
    /// **Note:** A Node cannot have a parent until after it has been inserted into a Tree.
    ///
    /// ```
    /// use id_tree::Node;
    ///
    /// let five: Node<i32> = Node::new(5);
    ///
    /// assert!(five.parent().is_none());
    /// ```
    ///
    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    ///
    /// Returns an immutable reference to a Vec containing the NodeIds of this Node's children.
    ///
    /// **Note:** A Node cannot have any children until after it has been inserted into a Tree.
    ///
    /// ```
    /// use id_tree::Node;
    ///
    /// let six: Node<i32> = Node::new(6);
    ///
    /// assert_eq!(six.children().len(), 0);
    /// ```
    ///
    pub fn children(&self) -> &Vec<NodeId> {
        &self.children
    }
}

impl<T> MutableNode for Node<T> {
    fn set_parent(&mut self, parent: Option<NodeId>) {
        self.parent = parent;
    }

    fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    fn children_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.children
    }
}

#[cfg(test)]
mod node_tests {
    use super::Node;
    use super::super::NodeId;
    use super::super::snowflake::ProcessUniqueId;
    use super::super::MutableNode;

    #[test]
    fn test_new() {
        let node = Node::new(5);
        assert_eq!(node.children.capacity(), 0);
    }

    #[test]
    fn test_new_with_capacity() {
        let node = Node::new_with_child_capacity(5, 10);
        assert_eq!(node.children.capacity(), 10);
    }

    #[test]
    fn test_data() {
        let five = 5;
        let node = Node::new(five);
        assert_eq!(node.data(), &five);
    }

    #[test]
    fn test_data_mut() {
        let mut five = 5;
        let mut node = Node::new(five);
        assert_eq!(node.data_mut(), &mut five);
    }

    #[test]
    fn test_parent() {
        let mut node = Node::new(5);
        assert!(node.parent().is_none());

        let parent_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };

        node.set_parent(Some(parent_id));
        assert_eq!(node.parent(), Some(parent_id));
    }

    #[test]
    fn test_children() {
        let mut node = Node::new(5);
        assert_eq!(node.children().len(), 0);

        let child_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };
        node.add_child(child_id);

        assert_eq!(node.children().len(), 1);
        assert_eq!(node.children().get(0).unwrap(), &child_id);

        let mut node = Node::new(5);
        assert_eq!(node.children().len(), 0);

        let child_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0,
        };
        node.children_mut().push(child_id);

        assert_eq!(node.children().len(), 1);
        assert_eq!(node.children().get(0).unwrap(), &child_id);
    }
}
