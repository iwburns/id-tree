pub type NodeId = usize;

pub struct TreeBuilder<T> {
    root: Option<Node<T>>,
    capacity: usize
}

impl<T> TreeBuilder<T> {
    pub fn new() -> TreeBuilder<T> {
        TreeBuilder {
            root: None,
            capacity: 0
        }
    }

    pub fn with_root(&mut self, root: Node<T>) -> TreeBuilder<T> {
        TreeBuilder {
            root: Some(root),
            capacity: self.capacity
        }
    }

    pub fn with_capacity(&mut self, capacity: usize) -> TreeBuilder<T> {
        TreeBuilder {
            root: self.root.take(),
            capacity: capacity
        }
    }

    pub fn build(&mut self) -> Tree<T> {

        let mut tree = Tree {
            root: None,
            nodes: Vec::with_capacity(self.capacity),
            free_ids: Vec::new() //todo: should this start with capacity too?
        };

        if self.root.is_some() {
            tree.nodes.push(self.root.take());
            tree.root = Some(0);
        }

        tree
    }
}

pub struct Tree<T> {
    root: Option<NodeId>,
    nodes: Vec<Option<Node<T>>>,
    free_ids: Vec<NodeId>
}

pub struct Node<T> {
    data: T,
    parent: Option<NodeId>,
    children: Vec<NodeId>
}

impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Node {
            data: data,
            parent: None,
            children: Vec::new()
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

    fn set_parent(&mut self, parent: NodeId) {
        self.parent = Some(parent);
    }

    fn add_child(&mut self, child: NodeId) {
        self.children.push(child);
    }

    fn children_mut(&mut self) -> &mut Vec<NodeId> {
        &mut self.children
    }
}

#[cfg(test)]
mod tree_builder_tests {
    use super::TreeBuilder;
    use super::Node;
    use super::NodeId;

    #[test]
    fn test_new() {
        let tb: TreeBuilder<i32> = TreeBuilder::new();
        assert!(tb.root.is_none());
        assert_eq!(tb.capacity, 0);
    }

    #[test]
    fn test_with_root() {
        let tb: TreeBuilder<i32> = TreeBuilder::new()
            .with_root(Node::new(5));

        assert_eq!(tb.root.unwrap().data(), &5);
        assert_eq!(tb.capacity, 0);
    }

    #[test]
    fn test_with_capacity() {
        let tb: TreeBuilder<i32> = TreeBuilder::new()
            .with_capacity(10);

        assert!(tb.root.is_none());
        assert_eq!(tb.capacity, 10);
    }

    #[test]
    fn test_with_root_with_capacity() {
        let tb: TreeBuilder<i32> = TreeBuilder::new()
            .with_root(Node::new(5))
            .with_capacity(10);

        assert_eq!(tb.root.unwrap().data(), &5);
        assert_eq!(tb.capacity, 10);
    }
}

#[cfg(test)]
mod node_tests {
    use super::Node;
    use super::NodeId;

    #[test]
    fn test_new() {
        let _node = Node::new(5);
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
        let node = Node::new(5);
        assert_eq!(node.parent(), None);

        let parent_id: NodeId = 0;
        let mut node = Node::new(5);
        node.set_parent(parent_id);

        assert_eq!(node.parent(), Some(parent_id))
    }

    #[test]
    fn test_children() {
        let node = Node::new(5);
        assert_eq!(node.children().len(), 0);

        let mut node = Node::new(5);
        let child_id: NodeId = 2;
        node.add_child(child_id);
        let children = node.children();

        assert_eq!(children.len(), 1);
        assert_eq!(children.get(0).unwrap(), &child_id);

        let mut node = Node::new(5);
        let child_id: NodeId = 2;
        node.children_mut().push(child_id);
        let children = node.children();

        assert_eq!(children.len(), 1);
        assert_eq!(children.get(0).unwrap(), &child_id);
    }
}
