use snowflake::ProcessUniqueId;
use ::*;

///
/// A `OptTree` builder that provides more control over how a `OptTree` is created.
///
pub struct OptTreeBuilder<T> {
    root: Option<OptNode<T>>,
    node_capacity: usize,
    swap_capacity: usize,
}

impl<T> OptTreeBuilder<T> {
    ///
    /// Creates a new `OptTreeBuilder` with the default settings.
    ///
    /// ```
    /// use id_tree::OptTreeBuilder;
    ///
    /// let _tree_builder: OptTreeBuilder<i32> = OptTreeBuilder::new();
    /// ```
    ///
    pub fn new() -> OptTreeBuilder<T> {
        OptTreeBuilder {
            root: None,
            node_capacity: 0,
            swap_capacity: 0,
        }
    }

    ///
    /// Sets the root `Node` of the `OptTreeBuilder`.
    ///
    /// ```
    /// use id_tree::OptTreeBuilder;
    /// use id_tree::Node;
    /// use id_tree::OptNode;
    ///
    /// let _tree_builder = OptTreeBuilder::new().with_root(OptNode::new(1));
    /// ```
    ///
    pub fn with_root(mut self, root: OptNode<T>) -> OptTreeBuilder<T> {
        self.root = Some(root);
        self
    }

    ///
    /// Sets the node_capacity of the `OptTreeBuilder`.
    ///
    /// Since `OptTree`s own their `OptNode`s, they must allocate storage space as `OptNode`s are
    /// inserted. Using this setting allows the `OptTree` to pre-allocate space for `OptNode`s
    /// ahead of time, so that the space allocations don't happen as the `OptNode`s are inserted.
    ///
    /// _Use of this setting is recommended if you know the **maximum number** of `OptNode`s that
    /// your `Tree` will **contain** at **any given time**._
    ///
    /// ```
    /// use id_tree::OptTreeBuilder;
    ///
    /// let _tree_builder: OptTreeBuilder<i32> = OptTreeBuilder::new().with_node_capacity(3);
    /// ```
    ///
    pub fn with_node_capacity(mut self, node_capacity: usize) -> OptTreeBuilder<T> {
        self.node_capacity = node_capacity;
        self
    }

    ///
    /// Sets the swap_capacity of the `OptTreeBuilder`.
    ///
    /// This is important because `OptTree`s attempt to save time by re-using storage space when
    /// `OptNode`s are removed (instead of shuffling `OptNode`s around internally).  To do this,
    /// the `OptTree` must store information about the space left behind when a `OptNode` is
    /// removed. Using this setting allows the `OptTree` to pre-allocate this storage space
    /// instead of doing so as `OptNode`s are removed from the `OptTree`.
    ///
    /// _Use of this setting is recommended if you know the **maximum "net number of
    /// removals"** that have occurred **at any given time**._
    ///
    /// For example:
    /// ---
    /// In **Scenario 1**:
    ///
    /// * Add 3 `OptNode`s, Remove 2 `OptNode`s, Add 1 `OptNode`.
    ///
    /// The most amount of nodes that have been removed at any given time is **2**.
    ///
    /// But in **Scenario 2**:
    ///
    /// * Add 3 `OptNode`s, Remove 2 `OptNode`s, Add 1 `OptNode`, Remove 2 `OptNode`s.
    ///
    /// The most amount of nodes that have been removed at any given time is **3**.
    ///
    /// ```
    /// use id_tree::OptTreeBuilder;
    ///
    /// let _tree_builder: OptTreeBuilder<i32> = OptTreeBuilder::new().with_swap_capacity(3);
    /// ```
    ///
    pub fn with_swap_capacity(mut self, swap_capacity: usize) -> OptTreeBuilder<T> {
        self.swap_capacity = swap_capacity;
        self
    }

    ///
    /// Build a `OptTree` based upon the current settings in the `OptTreeBuilder`.
    ///
    /// ```
    /// use id_tree::OptTreeBuilder;
    /// use id_tree::OptTree;
    /// use id_tree::Node;
    /// use id_tree::OptNode;
    ///
    /// let _tree: OptTree<i32> = OptTreeBuilder::new()
    ///         .with_root(OptNode::new(5))
    ///         .with_node_capacity(3)
    ///         .with_swap_capacity(2)
    ///         .build();
    /// ```
    ///
    pub fn build(mut self) -> OptTree<T> {

        let tree_id = ProcessUniqueId::new();

        let mut tree = OptTree {
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

pub struct OptTree<T> {
    id: ProcessUniqueId,
    root: Option<NodeId>,
    pub(crate) nodes: Vec<Option<OptNode<T>>>,
    free_ids: Vec<NodeId>,
}
//
//impl<'a, T: 'a> Tree<'a, T> for OptTree<T> {
//
//}
