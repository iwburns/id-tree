use super::snowflake::ProcessUniqueId;
use super::Node;
use super::NodeId;
use super::MutableNode;

//todo: remove panic!()s and replace with custom errors and return Result values instead

//todo: see if we can avoid bounds checks since we are managing the Ids manually here anyway.
//todo: I believe, theoretically, there should only be bounds checks happening in is_valid_node_id().

///
/// A `Tree` builder that provides more control over how a `Tree` is created.
///
pub struct TreeBuilder<T> {
    root: Option<Node<T>>,
    node_capacity: usize,
    swap_capacity: usize,
}

impl<T> TreeBuilder<T> {
    ///
    /// Creates a new `TreeBuilder` with the default settings.
    ///
    /// ```
    /// use id_tree::TreeBuilder;
    ///
    /// let _tree_builder: TreeBuilder<i32> = TreeBuilder::new();
    /// ```
    ///
    pub fn new() -> TreeBuilder<T> {
        TreeBuilder {
            root: None,
            node_capacity: 0,
            swap_capacity: 0,
        }
    }

    ///
    /// Sets the root `Node` of the `TreeBuilder`.
    ///
    /// ```
    /// use id_tree::TreeBuilder;
    /// use id_tree::Node;
    ///
    /// let _tree_builder = TreeBuilder::new().with_root(Node::new(1));
    /// ```
    ///
    pub fn with_root(mut self, root: Node<T>) -> TreeBuilder<T> {
        self.root = Some(root);
        self
    }

    ///
    /// Sets the node_capacity of the `TreeBuilder`.
    ///
    /// Since `Tree`s own their `Node`s, they must allocate storage space as `Node`s are inserted.
    /// Using this setting allows the `Tree` to pre-allocate space for `Node`s ahead of time, so
    /// that the space allocations don't happen as the `Node`s are inserted.
    ///
    /// _Use of this setting is recommended if you know the **maximum number** of `Node`s that your
    /// `Tree` will **contain** at **any given time**._
    ///
    /// ```
    /// use id_tree::TreeBuilder;
    ///
    /// let _tree_builder: TreeBuilder<i32> = TreeBuilder::new().with_node_capacity(3);
    /// ```
    ///
    pub fn with_node_capacity(mut self, node_capacity: usize) -> TreeBuilder<T> {
        self.node_capacity = node_capacity;
        self
    }

    ///
    /// Sets the swap_capacity of the `TreeBuilder`.
    ///
    /// This is important because `Tree`s attempt to save time by re-using storage space when `Node`s
    /// are removed (instead of shuffling `Node`s around internally).  To do this, the `Tree` must
    /// store information about the space left behind when a `Node` is removed. Using this setting
    /// allows the `Tree` to pre-allocate this storage space instead of doing so as `Node`s are
    /// removed from the `Tree`.
    ///
    /// _Use of this setting is recommended if you know the **maximum "net number of
    /// removals"** that have occurred **at any given time**._
    ///
    /// For example:
    /// ---
    /// In **Scenario 1**:
    ///
    /// * Add 3 `Node`s, Remove 2 `Node`s, Add 1 `Node`.
    ///
    /// The most amount of nodes that have been removed at any given time is **2**.
    ///
    /// But in **Scenario 2**:
    ///
    /// * Add 3 `Node`s, Remove 2 `Node`s, Add 1 `Node`, Remove 2 `Node`s.
    ///
    /// The most amount of nodes that have been removed at any given time is **3**.
    ///
    /// ```
    /// use id_tree::TreeBuilder;
    ///
    /// let _tree_builder: TreeBuilder<i32> = TreeBuilder::new().with_swap_capacity(3);
    /// ```
    ///
    pub fn with_swap_capacity(mut self, swap_capacity: usize) -> TreeBuilder<T> {
        self.swap_capacity = swap_capacity;
        self
    }

    ///
    /// Build a `Tree` based upon the current settings in the `TreeBuilder`.
    ///
    /// ```
    /// use id_tree::TreeBuilder;
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let _tree: Tree<i32> = TreeBuilder::new()
    ///         .with_root(Node::new(5))
    ///         .with_node_capacity(3)
    ///         .with_swap_capacity(2)
    ///         .build();
    /// ```
    ///
    pub fn build(mut self) -> Tree<T> {

        let tree_id = ProcessUniqueId::new();

        let mut tree = Tree {
            id: tree_id,
            root: None,
            nodes: Vec::with_capacity(self.node_capacity),
            free_ids: Vec::with_capacity(self.swap_capacity),
        };

        if self.root.is_some() {

            let node_id = NodeId {
                tree_id: tree_id,
                index: 0,
            };

            tree.nodes.push(self.root.take());
            tree.root = Some(node_id);
        }

        tree
    }
}

//todo: add more data here.
///
/// A tree structure consisting of `Node`s.
///
pub struct Tree<T> {
    id: ProcessUniqueId,
    root: Option<NodeId>,
    nodes: Vec<Option<Node<T>>>,
    free_ids: Vec<NodeId>,
}

impl<T> Tree<T> {

    ///
    /// Creates a new `Tree` with default settings.
    ///
    /// ```
    /// use id_tree::Tree;
    ///
    /// let _tree: Tree<i32> = Tree::new();
    /// ```
    ///
    pub fn new() -> Tree<T> {
        TreeBuilder::new().build()
    }

    ///
    /// Sets the root of the `Tree`.
    ///
    /// If there is already a root `Node` present in the tree, that `Node` is set as the first child
    /// of the new root.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    ///
    /// tree.set_root(Node::new(5));
    /// ```
    ///
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

    ///
    /// Add a new `Node` to the tree as the child of a `Node` specified by the given `NodeId`.
    /// Returns the `NodeId` of the child that was added.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(1);
    /// let child_node = Node::new(2);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// tree.add_child(root_id, child_node);
    /// ```
    ///
    pub fn add_child(&mut self, parent_id: NodeId, child: Node<T>) -> NodeId {
        if !self.is_valid_node_id(parent_id) {
            //todo: is panic the right tool here?
            // maybe having this return Result would be better.
            panic!("Invalid NodeId given for parent_id.");
        }

        let new_child_id = self.insert_new_node(child);
        self.set_as_parent_and_child(parent_id, new_child_id);

        new_child_id
    }

    ///
    /// Get an immutable reference to a `Node`.
    ///
    /// If the `NodeId` provided is invalid (whether the
    /// `Node` in question has already been removed, or the `NodeId` belongs to a different `Tree`),
    /// this function returns a None value.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(5));
    ///
    /// let root_node: &Node<i32> = tree.get(root_id).unwrap();
    /// ```
    ///
    pub fn get(&self, node_id: NodeId) -> Option<&Node<T>> {
        if self.is_valid_node_id(node_id) {
            return (*self.nodes.get(node_id.index).unwrap()).as_ref();
        }
        None
    }

    ///
    /// Get a mutable reference to a `Node`.
    ///
    /// If the `NodeId` provided is invalid (whether the
    /// `Node` in question has already been removed, or the `NodeId` belongs to a different `Tree`),
    /// this function returns a None value.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(5));
    ///
    /// let root_node: &mut Node<i32> = tree.get_mut(root_id).unwrap();
    /// ```
    ///
    pub fn get_mut(&mut self, node_id: NodeId) -> Option<&mut Node<T>> {
        if self.is_valid_node_id(node_id) {
            return (*self.nodes.get_mut(node_id.index).unwrap()).as_mut();
        }
        None
    }

    ///
    /// Remove a `Node` from the `Tree` and return it while dropping all of its children
    /// from the `Tree`.
    ///
    /// The `Node` that is returned will have its children cleared because those `NodeId`s are no
    /// longer valid.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(1);
    /// let child_node = Node::new(2);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// tree.add_child(root_id, child_node);
    ///
    /// let root_node = tree.remove_node_drop_children(root_id);
    /// ```
    ///
    pub fn remove_node_drop_children(&mut self, node_id: NodeId) -> Node<T> {
        if !self.is_valid_node_id(node_id) {
            //todo: is panic the right tool here?
            // maybe having this return Result would be better.
            panic!("Invalid NodeId given for node_id.");
        }

        if self.is_root_node(node_id) {
            self.root = None;
        }

        self.drop_children_recursive(node_id);

        let mut node = self.remove_node(node_id);
        //clear children because they're no longer valid
        node.children_mut().clear();

        node
    }

    ///
    /// Remove a `Node` from the `Tree` and return it while leaving all of its children in the
    /// `Tree`.
    ///
    /// The `Node` that is returned will maintain its children because those `NodeId`s are still
    /// valid. These orphaned `Node`s will remain in memory until the `Tree` goes out of scope.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(1);
    /// let child_node = Node::new(2);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// tree.add_child(root_id, child_node);
    ///
    /// let root_node = tree.remove_node_orphan_children(root_id);
    /// ```
    ///
    pub fn remove_node_orphan_children(&mut self, node_id: NodeId) -> Node<T> {
        if !self.is_valid_node_id(node_id) {
            //todo: is panic the right tool here?
            // maybe having this return Result would be better.
            panic!("Invalid NodeId given for node_id.");
        }

        self.remove_node(node_id)
    }

    ///
    /// Returns the `NodeId` of the root `Node` if it exists.  Otherwise a None value is returned.
    ///
    /// This might be useful if you only ever want to traverse the tree in its entirety.  In that
    /// situation, you can throw away all of the `NodeId`s after the tree has been constructed since
    /// you can always find the root `Node`'s `NodeId`.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(5));
    ///
    /// assert_eq!(root_id, tree.root_node_id().unwrap());
    /// ```
    ///
    pub fn root_node_id(&self) -> Option<NodeId> {
        self.root
    }

    fn set_as_parent_and_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        self.get_mut(parent_id)
            .expect("parent_id refers to a None value.")
            .add_child(child_id);

        self.get_mut(child_id)
            .expect("child_id refers to a None value.")
            .set_parent(Some(parent_id));
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

    fn remove_node(&mut self, node_id: NodeId) -> Node<T> {

        let node = self.remove_node_dirty(node_id);

        //todo: it seems like I might be missing an edge case here, but I'm not sure what it is
        if let Some(parent_id) = node.parent() {
            if let Some(parent_node) = self.get_mut(parent_id) {
                parent_node.children_mut().retain(|&child_id| child_id != node_id);
            } else {
                panic!("Invalid parent_id for node_id: {:?}", node_id);
            }
        }

        node
    }

    fn remove_node_dirty(&mut self, node_id: NodeId) -> Node<T> {
        debug_assert!(self.is_valid_node_id(node_id), "Invalid node_id found in what should be a 'protected' function.");

        self.nodes.push(None);
        let node = self.nodes.swap_remove(node_id.index).expect("node_id refers to a None value even though it is should be valid.");
        self.free_ids.push(node_id);

        node
    }

    fn drop_children_recursive(&mut self, node_id: NodeId) {

        //todo: is there a way to avoid this clone?
        let children = self.get(node_id).unwrap().children().clone();

        for child_id in children {
            self.drop_children_recursive(child_id);
            self.remove_node_dirty(child_id);
        }
    }

    fn new_node_id(&self, node_index: usize) -> NodeId {
        NodeId {
            tree_id: self.id,
            index: node_index,
        }
    }

    fn is_valid_node_id(&self, node_id: NodeId) -> bool {
        if node_id.tree_id != self.id {
            //the node_id belongs to a different tree.
            return false;
        }

        let optional_node = self.nodes.get(node_id.index);

        if optional_node.is_none() {
            //the index is out of bounds;
            return false;
        }

        if optional_node.unwrap().is_none() {
            // the node at that index was removed.
            return false;
        }

        true
    }

    fn is_root_node(&self, node_id: NodeId) -> bool {
        match self.root {
            Some(root_id) => {
                root_id == node_id
            },
            None => false
        }
    }
}

#[cfg(test)]
mod tree_builder_tests {
    use super::TreeBuilder;
    use super::super::Node;

    #[test]
    fn test_new() {
        let tb: TreeBuilder<i32> = TreeBuilder::new();
        assert!(tb.root.is_none());
        assert_eq!(tb.node_capacity, 0);
        assert_eq!(tb.swap_capacity, 0);
    }

    #[test]
    fn test_with_root() {
        let tb: TreeBuilder<i32> = TreeBuilder::new()
            .with_root(Node::new(5));

        assert_eq!(tb.root.unwrap().data(), &5);
        assert_eq!(tb.node_capacity, 0);
        assert_eq!(tb.swap_capacity, 0);
    }

    #[test]
    fn test_with_node_capacity() {
        let tb: TreeBuilder<i32> = TreeBuilder::new()
            .with_node_capacity(10);

        assert!(tb.root.is_none());
        assert_eq!(tb.node_capacity, 10);
        assert_eq!(tb.swap_capacity, 0);
    }

    #[test]
    fn test_with_swap_capacity() {
        let tb: TreeBuilder<i32> = TreeBuilder::new()
            .with_swap_capacity(10);

        assert!(tb.root.is_none());
        assert_eq!(tb.node_capacity, 0);
        assert_eq!(tb.swap_capacity, 10);
    }

    #[test]
    fn test_with_all_settings() {
        let tb: TreeBuilder<i32> = TreeBuilder::new()
            .with_root(Node::new(5))
            .with_node_capacity(10)
            .with_swap_capacity(3);

        assert_eq!(tb.root.unwrap().data(), &5);
        assert_eq!(tb.node_capacity, 10);
        assert_eq!(tb.swap_capacity, 3);
    }

    #[test]
    fn test_build() {
        let tree = TreeBuilder::new()
            .with_root(Node::new(5))
            .with_node_capacity(10)
            .with_swap_capacity(3)
            .build();

        let root = tree.get(tree.root_node_id().unwrap()).unwrap();

        assert_eq!(root.data(), &5);
        assert_eq!(tree.nodes.capacity(), 10);
        assert_eq!(tree.free_ids.capacity(), 3);
    }
}

#[cfg(test)]
mod tree_tests {
    use super::Tree;
    use super::TreeBuilder;
    use super::super::NodeId;
    use super::super::Node;

    #[test]
    fn test_new() {
        let tree: Tree<i32> = Tree::new();

        assert_eq!(tree.root, None);
        assert_eq!(tree.nodes.len(), 0);
        assert_eq!(tree.free_ids.len(), 0);
    }


    #[test]
    fn test_get() {
        let tree = TreeBuilder::new().with_root(Node::new(5)).build();

        let root = tree.get(tree.root.unwrap()).unwrap();

        assert_eq!(root.data(), &5);
    }

    #[test]
    fn test_get_mut() {
        let mut tree = TreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.root.unwrap();

        {
            let root = tree.get(root_id).unwrap();
            assert_eq!(root.data(), &5);
        }

        {
            let root = tree.get_mut(root_id).unwrap();
            *root.data_mut() = 6;
        }

        let root = tree.get(root_id).unwrap();
        assert_eq!(root.data(), &6);
    }

    #[test]
    fn test_set_root() {
        let a = 5;
        let b = 6;
        let node_a = Node::new(a);
        let node_b = Node::new(b);

        let mut tree = TreeBuilder::new().build();

        let node_a_id = tree.set_root(node_a);
        let root_id = tree.root.unwrap();
        assert_eq!(node_a_id, root_id);

        {
            let node_a_ref = tree.get(node_a_id).unwrap();
            let root_ref = tree.get(root_id).unwrap();
            assert_eq!(node_a_ref.data(), &a);
            assert_eq!(root_ref.data(), &a);
        }

        let node_b_id = tree.set_root(node_b);
        let root_id = tree.root.unwrap();
        assert_eq!(node_b_id, root_id);

        {
            let node_b_ref = tree.get(node_b_id).unwrap();
            let root_ref = tree.get(root_id).unwrap();
            assert_eq!(node_b_ref.data(), &b);
            assert_eq!(root_ref.data(), &b);

            let node_b_child_id = node_b_ref.children().get(0).unwrap();
            let node_b_child_ref = tree.get(*node_b_child_id).unwrap();
            assert_eq!(node_b_child_ref.data(), &a);
        }
    }

    #[test]
    fn test_root_node_id() {
        let tree = TreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.root.unwrap();
        let root_node_id = tree.root_node_id().unwrap();

        assert_eq!(root_id, root_node_id);
    }

    #[test]
    fn test_add_child() {
        let a = 1;
        let b = 2;
        let r = 5;

        let mut tree = TreeBuilder::new()
            .with_root(Node::new(r))
            .build();

        let node_a = Node::new(a);
        let node_b = Node::new(b);

        let root_id = tree.root.unwrap();
        let node_a_id = tree.add_child(root_id, node_a);
        let node_b_id = tree.add_child(root_id, node_b);

        let node_a_ref = tree.get(node_a_id).unwrap();
        let node_b_ref = tree.get(node_b_id).unwrap();
        assert_eq!(node_a_ref.data(), &a);
        assert_eq!(node_b_ref.data(), &b);

        assert_eq!(node_a_ref.parent().unwrap(), root_id);
        assert_eq!(node_b_ref.parent().unwrap(), root_id);

        let root_node_ref = tree.get(root_id).unwrap();
        let root_children: &Vec<NodeId> = root_node_ref.children();

        let child_1_id = root_children.get(0).unwrap();
        let child_2_id = root_children.get(1).unwrap();

        let child_1_ref = tree.get(*child_1_id).unwrap();
        let child_2_ref = tree.get(*child_2_id).unwrap();

        assert_eq!(child_1_ref.data(), &a);
        assert_eq!(child_2_ref.data(), &b);
    }

    #[test]
    fn test_remove_node_drop_children() {

        let mut tree = TreeBuilder::new()
            .with_root(Node::new(5))
            .build();

        let root_id = tree.root.unwrap();

        let node_1_id = tree.add_child(root_id, Node::new(1));
        let node_2_id = tree.add_child(node_1_id, Node::new(2));
        let node_3_id = tree.add_child(node_1_id, Node::new(3));

        let node_1 = tree.remove_node_drop_children(node_1_id);

        assert_eq!(node_1.data(), &1);
        assert_eq!(node_1.children().len(), 0);
        assert_eq!(node_1.parent().unwrap(), root_id);
        assert!(tree.get(node_1_id).is_none());
        assert!(tree.get(node_2_id).is_none());
        assert!(tree.get(node_3_id).is_none());
    }

    #[test]
    fn test_remove_node_orphan_children() {

        let mut tree = TreeBuilder::new()
            .with_root(Node::new(5))
            .build();

        let root_id = tree.root.unwrap();

        let node_1_id = tree.add_child(root_id, Node::new(1));
        let node_2_id = tree.add_child(node_1_id, Node::new(2));
        let node_3_id = tree.add_child(node_1_id, Node::new(3));

        let node_1 = tree.remove_node_orphan_children(node_1_id);

        assert_eq!(node_1.data(), &1);
        assert_eq!(node_1.children().len(), 2);
        assert_eq!(node_1.parent().unwrap(), root_id);
        assert!(tree.get(node_1_id).is_none());
        assert_eq!(tree.get(node_2_id).unwrap().data(), &2);
        assert_eq!(tree.get(node_3_id).unwrap().data(), &3);
    }
}
