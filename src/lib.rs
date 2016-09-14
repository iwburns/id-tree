extern crate snowflake;
use self::snowflake::ProcessUniqueId;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: usize
}

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

        let tree_id = ProcessUniqueId::new();

        let mut tree = Tree {
            id: tree_id,
            root: None,
            nodes: Vec::with_capacity(self.capacity),
            free_ids: Vec::new() //todo: should this start with capacity too?
        };

        if self.root.is_some() {

            let node_id = NodeId {
                tree_id: tree_id,
                index: 0
            };

            tree.nodes.push(self.root.take());
            tree.root = Some(node_id);
        }

        tree
    }
}

pub struct Tree<T> {
    id: ProcessUniqueId,
    root: Option<NodeId>,
    nodes: Vec<Option<Node<T>>>,
    free_ids: Vec<NodeId>
}

impl<T> Tree<T> {
    pub fn set_root(&mut self, new_root: Node<T>) -> NodeId {
        let new_root_id = self.insert_new_node(new_root);

        match self.root {
            Some(current_root_node_id) => {
                self.set_as_parent_and_child(new_root_id, current_root_node_id);
            },
            None => ()
        };

        self.root = Some(new_root_id);
        new_root_id
    }

    pub fn get(&self, node_id: NodeId) -> Option<&Node<T>> {
        if self.is_valid_node_id(node_id) {
            return (*self.nodes.get(node_id.index).unwrap()).as_ref();
        }
        None
    }

    pub fn get_mut(&mut self, node_id: NodeId) -> Option<&mut Node<T>> {
        if self.is_valid_node_id(node_id) {
            return (*self.nodes.get_mut(node_id.index).unwrap()).as_mut();
        }
        None
    }

    pub fn root_node_id(&self) -> Option<NodeId> {
        self.root
    }

    fn set_as_parent_and_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        self.get_mut(parent_id)
            .expect("parent_id refers to a None value.")
            .add_child(child_id);

        self.get_mut(child_id)
            .expect("child_id refers to a None value.")
            .set_parent(parent_id);
    }

    fn insert_new_node(&mut self, new_node: Node<T>) -> NodeId {

        if self.free_ids.len() > 0 {
            let new_node_id: NodeId = self.free_ids.pop()
                .expect("Couldn't pop from Vec with len() > 0 while inserting a new node.");

            self.nodes.push(Some(new_node));
            self.nodes.swap_remove(new_node_id.index);
            return new_node_id;

        } else {
            let new_node_index = self.nodes.len();
            self.nodes.push(Some(new_node));

            return self.new_node_id(new_node_index);
        }
    }

    fn new_node_id(&self, node_index: usize) -> NodeId {
        NodeId {
            tree_id: self.id,
            index: node_index
        }
    }

    fn is_valid_node_id(&self, node_id: NodeId) -> bool {
        if node_id.tree_id != self.id {
            //the node_id belongs to a different tree.
            return false;
        }

        if self.nodes.get(node_id.index).is_none() {
            //the index is out of bounds
            return false;
        }

        if self.nodes.get(node_id.index).unwrap().is_none() {
            //the node at that index was removed.
            return false;
        }

        true
    }
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
mod tree_tests {
    use super::Tree;
    use super::TreeBuilder;
    use super::NodeId;
    use super::Node;

    #[test]
    fn test_get() {
        let root_node = Node::new(5);
        let mut tree = TreeBuilder::new().build();

        let root_id = tree.set_root(root_node);

        let root_node_ref = tree.get(root_id).unwrap();
        assert_eq!(root_node_ref.data(), &5);
    }

    #[test]
    fn test_get_mut() {
        let root_node = Node::new(5);
        let mut tree = TreeBuilder::new().build();

        let root_id = tree.set_root(root_node);

        {
            let root_node_ref: &mut Node<i32> = tree.get_mut(root_id).unwrap();
            assert_eq!(root_node_ref.data(), &5);
        }

        {
            let root_node_ref: &mut Node<i32> = tree.get_mut(root_id).unwrap();
            *root_node_ref.data_mut() = 6;
        }

        let root_node_ref = tree.get_mut(root_id).unwrap();
        assert_eq!(root_node_ref.data(), &6);
    }

    #[test]
    fn test_set_root() {
        let node_a = Node::new(5);
        let node_b = Node::new(6);
        let mut tree = TreeBuilder::new().build();

        let node_a_id = tree.set_root(node_a);
        let root_id = tree.root_node_id().unwrap();
        assert_eq!(node_a_id, root_id);
        {
            let node_a_ref = tree.get(node_a_id).unwrap();
            let root_ref = tree.get(root_id).unwrap();
            assert_eq!(node_a_ref.data(), &5);
            assert_eq!(root_ref.data(), &5);
        }

        let node_b_id = tree.set_root(node_b);
        let root_id = tree.root_node_id().unwrap();
        assert_eq!(node_b_id, root_id);
        {
            let node_b_ref = tree.get(node_b_id).unwrap();
            let root_ref = tree.get(root_id).unwrap();
            assert_eq!(node_b_ref.data(), &6);
            assert_eq!(root_ref.data(), &6);

            let node_b_child_id = node_b_ref.children().get(0).unwrap();
            let node_b_child_ref = tree.get(*node_b_child_id).unwrap();
            assert_eq!(node_b_child_ref.data(), &5);
        }
    }

    #[test]
    fn test_root_node_id() {
        let root_node = Node::new(5);
        let mut tree = TreeBuilder::new().build();

        let root_id = tree.set_root(root_node);
        let root_node_id = tree.root_node_id().unwrap();

        assert_eq!(root_id, root_node_id);
    }
}

#[cfg(test)]
mod node_tests {
    use super::Node;
    use super::NodeId;
    use super::snowflake::ProcessUniqueId;

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
        assert!(node.parent().is_none());

        let parent_id: NodeId = NodeId {
            tree_id: ProcessUniqueId::new(),
            index: 0
        };
        let mut node = Node::new(5);
        node.set_parent(parent_id);

        assert_eq!(node.parent(), Some(parent_id))
    }

    #[test]
    fn test_children() {
        let node = Node::new(5);
        assert_eq!(node.children().len(), 0);

        let tree_id = ProcessUniqueId::new();

        let mut node = Node::new(5);
        let child_id: NodeId = NodeId {
            tree_id: tree_id,
            index: 2
        };
        node.add_child(child_id);
        let children = node.children();

        assert_eq!(children.len(), 1);
        assert_eq!(children.get(0).unwrap(), &child_id);

        let mut node = Node::new(5);
        let child_id: NodeId = NodeId {
            tree_id: tree_id,
            index: 2
        };
        node.children_mut().push(child_id);
        let children = node.children();

        assert_eq!(children.len(), 1);
        assert_eq!(children.get(0).unwrap(), &child_id);
    }
}
