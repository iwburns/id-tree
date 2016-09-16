use super::NodeId;
use super::MutableNode;

pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node::new_with_child_capacity(data, 0)
    }

    //todo: make a NodeBuilder for this kind of thing
    pub fn new_with_child_capacity(data: T, capacity: usize) -> Node<T> {
        Node {
            data: data,
            parent: None,
            children: Vec::with_capacity(capacity),
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

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
