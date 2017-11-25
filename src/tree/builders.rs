use std::marker::PhantomData;

use ::*;
use super::core::CoreTree;

///
/// A `VecTree` builder that provides more control over how a `VecTree` is created.
///
pub struct VecTreeBuilder<T> {
    root: Option<VecNode<T>>,
    node_capacity: usize,
    swap_capacity: usize,
}

impl<'a, T> VecTreeBuilder<T> {
    ///
    /// Creates a new `VecTreeBuilder` with the default settings.
    ///
    /// ```
    /// use id_tree::VecTreeBuilder;
    ///
    /// let _tree_builder: VecTreeBuilder<i32> = VecTreeBuilder::new();
    /// ```
    ///
    pub fn new() -> VecTreeBuilder<T> {
        VecTreeBuilder {
            root: None,
            node_capacity: 0,
            swap_capacity: 0,
        }
    }

    ///
    /// Sets the root `Node` of the `VecTreeBuilder`.
    ///
    /// ```
    /// use id_tree::VecTreeBuilder;
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let _tree_builder = VecTreeBuilder::new().with_root(VecNode::new(1));
    /// ```
    ///
    pub fn with_root(mut self, root: VecNode<T>) -> VecTreeBuilder<T> {
        self.root = Some(root);
        self
    }

    ///
    /// Sets the node_capacity of the `VecTreeBuilder`.
    ///
    /// Since `VecTree`s own their `VecNode`s, they must allocate storage space as `VecNode`s are
    /// inserted. Using this setting allows the `VecTree` to pre-allocate space for `VecNode`s
    /// ahead of time, so that the space allocations don't happen as the `VecNode`s are inserted.
    ///
    /// _Use of this setting is recommended if you know the **maximum number** of `VecNode`s that
    /// your `Tree` will **contain** at **any given time**._
    ///
    /// ```
    /// use id_tree::VecTreeBuilder;
    ///
    /// let _tree_builder: VecTreeBuilder<i32> = VecTreeBuilder::new().with_node_capacity(3);
    /// ```
    ///
    pub fn with_node_capacity(mut self, node_capacity: usize) -> VecTreeBuilder<T> {
        self.node_capacity = node_capacity;
        self
    }

    ///
    /// Sets the swap_capacity of the `VecTreeBuilder`.
    ///
    /// This is important because `VecTree`s attempt to save time by re-using storage space when
    /// `VecNode`s are removed (instead of shuffling `VecNode`s around internally).  To do this,
    /// the `VecTree` must store information about the space left behind when a `VecNode` is
    /// removed. Using this setting allows the `VecTree` to pre-allocate this storage space
    /// instead of doing so as `VecNode`s are removed from the `VecTree`.
    ///
    /// _Use of this setting is recommended if you know the **maximum "net number of
    /// removals"** that have occurred **at any given time**._
    ///
    /// For example:
    /// ---
    /// In **Scenario 1**:
    ///
    /// * Add 3 `VecNode`s, Remove 2 `VecNode`s, Add 1 `VecNode`.
    ///
    /// The most amount of nodes that have been removed at any given time is **2**.
    ///
    /// But in **Scenario 2**:
    ///
    /// * Add 3 `VecNode`s, Remove 2 `VecNode`s, Add 1 `VecNode`, Remove 2 `VecNode`s.
    ///
    /// The most amount of nodes that have been removed at any given time is **3**.
    ///
    /// ```
    /// use id_tree::VecTreeBuilder;
    ///
    /// let _tree_builder: VecTreeBuilder<i32> = VecTreeBuilder::new().with_swap_capacity(3);
    /// ```
    ///
    pub fn with_swap_capacity(mut self, swap_capacity: usize) -> VecTreeBuilder<T> {
        self.swap_capacity = swap_capacity;
        self
    }

    ///
    /// Build a `VecTree` based upon the current settings in the `VecTreeBuilder`.
    ///
    /// ```
    /// use id_tree::VecTreeBuilder;
    /// use id_tree::VecTree;
    /// use id_tree::Node;
    /// use id_tree::VecNode;
    ///
    /// let _tree: VecTree<i32> = VecTreeBuilder::new()
    ///         .with_root(VecNode::new(5))
    ///         .with_node_capacity(3)
    ///         .with_swap_capacity(2)
    ///         .build();
    /// ```
    ///
    pub fn build(self) -> VecTree<'a, T> {
        VecTree {
            core_tree: CoreTree::new(self.root, self.node_capacity, self.swap_capacity),
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod vec_tree_builder_tests {
    use ::*;

    #[test]
    fn new() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new();
        assert!(tb.root.is_none());
        assert_eq!(tb.node_capacity, 0);
        assert_eq!(tb.swap_capacity, 0);
    }

    #[test]
    fn with_root() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new().with_root(Node::new(5));

        assert_eq!(tb.root.unwrap().data(), &5);
        assert_eq!(tb.node_capacity, 0);
        assert_eq!(tb.swap_capacity, 0);
    }

    #[test]
    fn with_node_capacity() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new().with_node_capacity(10);

        assert!(tb.root.is_none());
        assert_eq!(tb.node_capacity, 10);
        assert_eq!(tb.swap_capacity, 0);
    }

    #[test]
    fn with_swap_capacity() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new().with_swap_capacity(10);

        assert!(tb.root.is_none());
        assert_eq!(tb.node_capacity, 0);
        assert_eq!(tb.swap_capacity, 10);
    }

    #[test]
    fn with_all_settings() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new()
            .with_root(Node::new(5))
            .with_node_capacity(10)
            .with_swap_capacity(3);

        assert_eq!(tb.root.unwrap().data(), &5);
        assert_eq!(tb.node_capacity, 10);
        assert_eq!(tb.swap_capacity, 3);
    }

    #[test]
    fn build() {
        let tree = VecTreeBuilder::new()
            .with_root(Node::new(5))
            .with_node_capacity(10)
            .with_swap_capacity(3)
            .build();

        let root = tree.get(tree.root_node_id().unwrap()).unwrap();

        assert_eq!(root.data(), &5);
        assert_eq!(tree.core_tree.nodes.capacity(), 10);
        assert_eq!(tree.core_tree.free_ids.capacity(), 3);
    }
}