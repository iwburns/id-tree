use std::cmp::Ordering;

use super::snowflake::ProcessUniqueId;
use super::Node;
use super::NodeId;
use super::MutableNode;
use super::NodeIdError;

//todo: change Tree::get() and Tree::get_mut() to return Result instead of Option

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

///
/// A tree structure consisting of `Node`s.
///
/// # Panics
/// While it is highly unlikely, any function that takes a `NodeId` _can_ `panic`.  This, however,
/// should only happen due to improper `NodeId` management within `id_tree` and should have nothing
/// to do with the library user's code.
///
/// **If this ever happens please report the issue.** `Panic`s are not expected behavior for this
/// library, but they can happen due to bugs.
///
pub struct Tree<T> {
    id: ProcessUniqueId,
    root: Option<NodeId>,
    nodes: Vec<Option<Node<T>>>,
    free_ids: Vec<NodeId>,
}

impl<T> Tree<T> {

    ///
    /// Creates a new `Tree` with default settings (no root `Node` and no space pre-allocation).
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

        if let Some(current_root_node_id) = self.root.clone() {
            self.set_as_parent_and_child(&new_root_id, &current_root_node_id);
        }

        self.root = Some(new_root_id.clone());
        new_root_id
    }

    ///
    /// Add a new `Node` to the tree as the child of a `Node` specified by the given `NodeId`.
    ///
    /// Returns a `Result` containing the `NodeId` of the child that was added or a `NodeIdError` if
    /// one occurred.
    ///
    /// Note: Adds the new Node to the end of its children.
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
    /// tree.insert_with_parent(child_node, &root_id);
    /// ```
    ///
    pub fn insert_with_parent(&mut self, child: Node<T>, parent_id: &NodeId) -> Result<NodeId, NodeIdError> {
        let (is_valid, error) = self.is_valid_node_id(parent_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::insert_with_parent: Missing an error value on finding an invalid NodeId."));
        }

        let new_child_id = self.insert_new_node(child);
        self.set_as_parent_and_child(parent_id, &new_child_id);

        Result::Ok(new_child_id)
    }

    ///
    /// Get an immutable reference to a `Node`.
    ///
    /// If the `NodeId` provided is invalid (whether the
    /// `Node` in question has already been removed, or the `NodeId` belongs to a different `Tree`),
    /// this function returns a `None` value.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(5));
    ///
    /// let root_node: &Node<i32> = tree.get(&root_id).unwrap();
    /// ```
    ///
    pub fn get(&self, node_id: &NodeId) -> Option<&Node<T>> {
        let (is_valid, _) = self.is_valid_node_id(node_id);
        if is_valid {
            return Some(self.get_unsafe(node_id));
        }
        None
    }

    ///
    /// Get a mutable reference to a `Node`.
    ///
    /// If the `NodeId` provided is invalid (whether the
    /// `Node` in question has already been removed, or the `NodeId` belongs to a different `Tree`),
    /// this function returns a `None` value.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(5));
    ///
    /// let root_node: &mut Node<i32> = tree.get_mut(&root_id).unwrap();
    /// ```
    ///
    pub fn get_mut(&mut self, node_id: &NodeId) -> Option<&mut Node<T>> {
        let (is_valid, _) = self.is_valid_node_id(node_id);
        if is_valid {
            return Some(self.get_mut_unsafe(node_id));
        }
        None
    }

    ///
    /// Remove a `Node` from the `Tree` and move its children up one "level" in the `Tree` if
    /// possible.
    ///
    /// Returns a `Result` containing the removed `Node` or a `NodeIdError` if one occurred.
    ///
    /// In other words, this `Node`'s children will point to its parent as their parent instead of
    /// this `Node`.  In addition, this `Node`'s parent will have this `Node`'s children added as
    /// its own children.  If this `Node` has no parent, then calling this function is the
    /// equivalent of calling `remove_node_orphan_children`.
    ///
    /// **NOTE:** The `Node` that is returned will have its parent and child values cleared to avoid
    /// providing the caller with extra copies of `NodeId`s should the corresponding `Node`s be
    /// removed from the `Tree` at a later time.
    ///
    /// If the caller needs a copy of the parent or child `NodeId`s, they must `Clone` them before
    /// this `Node` is removed from the `Tree`.  Please see the
    /// [Potential `NodeId` Issues](struct.NodeId.html#potential-nodeid-issues) section
    /// of the `NodeId` documentation for more information on the implications of calling `Clone` on
    /// a `NodeId`.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(1);
    /// let child_node = Node::new(2);
    /// let grandchild_node = Node::new(3);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// let child_id = tree.insert_with_parent(child_node, &root_id).ok().unwrap();
    /// tree.insert_with_parent(grandchild_node, &root_id);
    ///
    /// let root_node = tree.remove_node_lift_children(child_id);
    /// ```
    ///
    pub fn remove_node_lift_children(&mut self, node_id: NodeId) -> Result<Node<T>, NodeIdError> {
        let (is_valid, error) = self.is_valid_node_id(&node_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::remove_node_lift_children: Missing an error value on finding an invalid NodeId."));
        }

        if let Some(parent_id) = self.get_unsafe(&node_id).parent().cloned() {
            //attach children to parent
            for child_id in self.get_unsafe(&node_id).children().clone() {
                self.set_as_parent_and_child(&parent_id, &child_id);
            }
        } else {
            self.clear_parent_of_children(&node_id);
        }

        Result::Ok(self.remove_node(node_id))
    }

    ///
    /// Remove a `Node` from the `Tree` and leave all of its children in the `Tree`.
    ///
    /// Returns a `Result` containing the removed `Node` or a `NodeIdError` if one occurred.
    ///
    /// **NOTE:** The `Node` that is returned will have its parent and child values cleared to avoid
    /// providing the caller with extra copies of `NodeId`s should the corresponding `Node`s be
    /// removed from the `Tree` at a later time.
    ///
    /// If the caller needs a copy of the parent or child `NodeId`s, they must `Clone` them before
    /// this `Node` is removed from the `Tree`.  Please see the
    /// [Potential `NodeId` Issues](struct.NodeId.html#potential-nodeid-issues) section
    /// of the `NodeId` documentation for more information on the implications of calling `Clone` on
    /// a `NodeId`.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(1);
    /// let child_node = Node::new(2);
    /// let grandchild_node = Node::new(3);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// let child_id = tree.insert_with_parent(child_node, &root_id).ok().unwrap();
    /// tree.insert_with_parent(grandchild_node, &root_id);
    ///
    /// let root_node = tree.remove_node_orphan_children(child_id);
    /// ```
    ///
    pub fn remove_node_orphan_children(&mut self, node_id: NodeId) -> Result<Node<T>, NodeIdError> {
        let (is_valid, error) = self.is_valid_node_id(&node_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::remove_node_orphan_children: Missing an error value on finding an invalid NodeId."));
        }

        self.clear_parent_of_children(&node_id);
        Result::Ok(self.remove_node(node_id))
    }

    ///
    /// Remove a `Node` from the `Tree` including all its children recursively.
    ///
    /// Returns a `Result` containing the removed `Node` or a `NodeIdError` if one occurred.
    ///
    /// **NOTE:** The `Node` that is returned will have its parent value cleared to avoid
    /// providing the caller with extra copies of `NodeId`s should the corresponding `Node`s be
    /// removed from the `Tree` at a later time.
    ///
    /// **NOTE:** Please keep in mind: Children of this `NodeId` *are removed during this method call*,
    /// so `NodeId`s that previously pointed to them will no longer be valid after calling this method.
    /// This means even without using `Clone` you might end up with copies of invalid Id's.
    /// Use with caution.
    ///
    /// If the caller needs a copy of the parent `NodeId`s, they must `Clone` them before
    /// this `Node` is removed from the `Tree`.  Please see the
    /// [Potential `NodeId` Issues](struct.NodeId.html#potential-nodeid-issues) section
    /// of the `NodeId` documentation for more information on the implications of calling `Clone` on
    /// a `NodeId`.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(1);
    /// let child_node = Node::new(2);
    /// let grandchild_node = Node::new(3);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// let child_id = tree.insert_with_parent(child_node, &root_id).ok().unwrap();
    /// # let grandchild_id =
    /// tree.insert_with_parent(grandchild_node, &child_id).unwrap();
    ///
    /// # let child_id_copy = child_id.clone();
    /// let child_node = tree.remove_node_drop_children(child_id);
    /// # assert!(tree.get(&root_id).is_some());
    /// # assert!(!tree.get(&child_id_copy).is_some());
    /// # assert!(!tree.get(&grandchild_id).is_some());
    /// ```
    ///
    pub fn remove_node_drop_children(&mut self, node_id: NodeId) -> Result<Node<T>, NodeIdError> {
        let (is_valid, error) = self.is_valid_node_id(&node_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::remove_node_drop_children: Missing an error value on finding an invalid NodeId."));
        }

        let mut children = self.get_mut_unsafe(&node_id).take_children();
        for child in children.drain(..) {
            try!(self.remove_node_drop_children(child));
        }
        Result::Ok(self.remove_node(node_id))
    }

    ///
    /// Moves a `Node` inside a `Tree` to a new parent leaving all children in their place.
    ///
    /// If the new parent (let's call it `B`) is a descendant of the `Node` being moved (`A`), then
    /// the direct child of `A` on the path from `A` to `B` will be shifted upwards to take the
    /// place of its parent (`A`).  All other children of `A` will be left alone, meaning they will
    /// travel with it down the `Tree`.
    ///
    /// Please note that during the "shift-up" part of the above scenario, the `Node` being shifted
    /// up will always be added as the last child of its new parent.
    ///
    /// Returns an empty `Result` containing a `NodeIdError` if one occurred on either provided `Id`.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(1);
    /// let first_child_node = Node::new(2);
    /// let second_child_node = Node::new(3);
    /// let grandchild_node = Node::new(4);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// let first_child_id  = tree.insert_with_parent(first_child_node,  &root_id).ok().unwrap();
    /// let second_child_id = tree.insert_with_parent(second_child_node, &root_id).ok().unwrap();
    /// let grandchild_id   = tree.insert_with_parent(grandchild_node, &first_child_id).ok().unwrap();
    ///
    /// tree.move_node_to_parent(&grandchild_id, &second_child_id).unwrap();
    ///
    /// # assert!(!tree.get(&first_child_id).unwrap().children().contains(&grandchild_id));
    /// # assert!(tree.get(&second_child_id).unwrap().children().contains(&grandchild_id));
    /// ```
    ///
    pub fn move_node_to_parent(&mut self, node_id: &NodeId, parent_id: &NodeId) -> Result<(), NodeIdError> {
        let (is_valid, error) = self.is_valid_node_id(node_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::move_node_to_parent: Missing an error value on finding an invalid NodeId."));
        }

        let (is_valid, error) = self.is_valid_node_id(parent_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::move_node_to_parent: Missing an error value on finding an invalid NodeId."));
        }

        if let Some(subtree_root_id) = self.find_subtree_root_between_ids(parent_id, node_id).cloned() {
            //node_id is above parent_id, this is a move "down" the tree.

            let root = self.root.clone();

            if root.as_ref() == Some(node_id) {
                //we're moving the root down the tree.
                //also we know the root exists

                //detach subtree_root from node
                self.get_mut_unsafe(node_id).children_mut().retain(|id| id != &subtree_root_id);

                //set subtree_root as Tree root.
                self.get_mut_unsafe(&subtree_root_id).set_parent(None);
                self.root = Some(subtree_root_id);

                self.set_as_parent_and_child(parent_id, node_id);

            } else {
                //we're moving some other node down the tree.

                if let Some(old_parent) = self.get_unsafe(node_id).parent().cloned() {
                    // detach from old parent
                    self.get_mut_unsafe(&old_parent).children_mut().retain(|id| id != node_id);
                    // connect old parent and subtree root
                    self.set_as_parent_and_child(&old_parent, &subtree_root_id);
                } else {
                    // node is orphaned, need to set subtree_root's parent to None (same as node's)
                    self.get_mut_unsafe(&subtree_root_id).set_parent(None);
                }
                // detach subtree_root from node
                self.get_mut_unsafe(node_id).children_mut().retain(|id| id != &subtree_root_id);

                self.set_as_parent_and_child(parent_id, node_id);
            }

        } else {
            //this is a move "across" or "up" the tree.

            // detach from old parent
            if let Some(old_parent) = self.get_unsafe(node_id).parent().cloned() {
                self.get_mut_unsafe(&old_parent).children_mut().retain(|id| id != node_id);
            }

            self.set_as_parent_and_child(parent_id, node_id);
        }

        Result::Ok(())
    }

    ///
    /// Sorts the children of one node, in-place, using compare to compare the nodes
    ///
    /// This sort is stable and O(n log n) worst-case but allocates approximately 2 * n where n is the length of children
    ///
    /// Returns an empty `Result` containing a `NodeIdError` if one occurred.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(100);
    /// let first_child = Node::new(1);
    /// let second_child = Node::new(2);
    /// let third_child = Node::new(0);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// tree.insert_with_parent(first_child, &root_id).unwrap();
    /// tree.insert_with_parent(second_child, &root_id).unwrap();
    /// tree.insert_with_parent(third_child, &root_id).unwrap();
    ///
    /// tree.sort_children_by(&root_id, |a, b| a.data().cmp(b.data()));
    ///
    /// # for (i, id) in tree.get(&root_id).unwrap().children().iter().enumerate() {
    /// #   assert_eq!(*tree.get(&id).unwrap().data(), i as i32);
    /// # }
    /// ```
    ///
    pub fn sort_children_by<F>(&mut self, node_id: &NodeId, mut compare: F) -> Result<(), NodeIdError>
        where F: FnMut(&Node<T>, &Node<T>) -> Ordering
    {
        let (is_valid, error) = self.is_valid_node_id(node_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::sort_children_by: Missing an error value on finding an invalid NodeId."));
        }

        let mut children = self.get_mut_unsafe(node_id).take_children();
        children.sort_by(|a, b| {
            compare(self.get_unsafe(a), self.get_unsafe(b))
        });
        self.get_mut_unsafe(node_id).set_children(children);

        Result::Ok(())
    }

    ///
    /// Sorts the children of one node, in-place, comparing their data
    ///
    /// This sort is stable and O(n log n) worst-case but allocates approximately 2 * n where n is the length of children
    ///
    /// Returns an empty `Result` containing a `NodeIdError` if one occurred.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(100);
    /// let first_child = Node::new(1);
    /// let second_child = Node::new(2);
    /// let third_child = Node::new(0);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// tree.insert_with_parent(first_child, &root_id).unwrap();
    /// tree.insert_with_parent(second_child, &root_id).unwrap();
    /// tree.insert_with_parent(third_child, &root_id).unwrap();
    ///
    /// tree.sort_children_by_data(&root_id);
    ///
    /// # for (i, id) in tree.get(&root_id).unwrap().children().iter().enumerate() {
    /// #   assert_eq!(*tree.get(&id).unwrap().data(), i as i32);
    /// # }
    /// ```
    ///
    pub fn sort_children_by_data(&mut self, node_id: &NodeId) -> Result<(), NodeIdError>
        where T: Ord
    {
        let (is_valid, error) = self.is_valid_node_id(node_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::sort_children: Missing an error value on finding an invalid NodeId."));
        }

        let mut children = self.get_mut_unsafe(node_id).take_children();
        children.sort_by_key(|a| {
            self.get_unsafe(a).data()
        });
        self.get_mut_unsafe(node_id).set_children(children);

        Result::Ok(())
    }

    ///
    /// Sorts the children of one node, in-place, using f to extract a key by which to order the sort by.
    ///
    /// This sort is stable and O(n log n) worst-case but allocates approximately 2 * n where n is the length of children
    ///
    /// Returns an empty `Result` containing a `NodeIdError` if one occurred.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(100);
    /// let first_child = Node::new(1);
    /// let second_child = Node::new(2);
    /// let third_child = Node::new(0);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// tree.insert_with_parent(first_child, &root_id).unwrap();
    /// tree.insert_with_parent(second_child, &root_id).unwrap();
    /// tree.insert_with_parent(third_child, &root_id).unwrap();
    ///
    /// tree.sort_children_by_key(&root_id, |x| x.data().clone());
    ///
    /// # for (i, id) in tree.get(&root_id).unwrap().children().iter().enumerate() {
    /// #   assert_eq!(*tree.get(&id).unwrap().data(), i as i32);
    /// # }
    /// ```
    ///
    pub fn sort_children_by_key<B, F>(&mut self, node_id: &NodeId, mut f: F) -> Result<(), NodeIdError>
        where B: Ord, F: FnMut(&Node<T>) -> B
    {
        let (is_valid, error) = self.is_valid_node_id(node_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::sort_children_by_key: Missing an error value on finding an invalid NodeId."));
        }

        let mut children = self.get_mut_unsafe(node_id).take_children();
        children.sort_by_key(|a| {
            f(self.get_unsafe(a))
        });
        self.get_mut_unsafe(node_id).set_children(children);

        Result::Ok(())
    }

    ///
    /// Swaps two `Node`s including their children given their `NodeId`s.
    ///
    /// Returns an empty `Result` containing a `NodeIdError` if one occurred on either provided `Id`.
    ///
    /// Notes:
    ///     - Both `NodeId`s are still valid after this process and are not swapped.
    ///     - Keeps the positions of the `Node`s in their parents children collection.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let root_node = Node::new(1);
    /// let first_child_node = Node::new(2);
    /// let second_child_node = Node::new(3);
    /// let grandchild_node = Node::new(4);
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(root_node);
    ///
    /// let first_child_id  = tree.insert_with_parent(first_child_node,  &root_id).ok().unwrap();
    /// let second_child_id = tree.insert_with_parent(second_child_node, &root_id).ok().unwrap();
    /// let grandchild_id   = tree.insert_with_parent(grandchild_node, &second_child_id).unwrap();
    ///
    /// tree.swap_sub_tree(&first_child_id, &grandchild_id).unwrap();
    ///
    /// assert!(tree.get(&second_child_id).unwrap().children().contains(&first_child_id));
    /// assert!(tree.get(&root_id).unwrap().children().contains(&grandchild_id));
    /// ```
    ///
    pub fn swap_sub_tree(&mut self, first_id: &NodeId, second_id: &NodeId) -> Result<(), NodeIdError> {
        let (is_valid, error) = self.is_valid_node_id(first_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::swap_sub_tree: Missing an error value on finding an invalid NodeId."));
        }

        let (is_valid, error) = self.is_valid_node_id(second_id);
        if !is_valid {
            return Result::Err(error.expect("Tree::swap_sub_tree: Missing an error value on finding an invalid NodeId."));
        }

        let lower_upper_test = self.find_subtree_root_between_ids(first_id, second_id)
            .map(|_| (first_id, second_id))
            .or(
                self.find_subtree_root_between_ids(second_id, first_id)
                .map(|_| (second_id, first_id))
            );

        if let Some((lower_id, upper_id)) = lower_upper_test {
            let upper_parent_id = self.get_unsafe(upper_id).parent().cloned();

            let lower_parent_id = {
                let lower = self.get_mut_unsafe(&lower_id);
                let lower_parent_id = lower.parent().unwrap().clone(); //lower is lower, so has a parent for sure

                if upper_parent_id.is_some() {
                    lower.set_parent(upper_parent_id.clone());
                } else {
                    lower.set_parent(None);
                }

                lower_parent_id
            };

            self.get_mut_unsafe(&lower_parent_id).children_mut().retain(|x| x != lower_id);

            if upper_parent_id.is_some() {
                self.get_mut_unsafe(upper_parent_id.as_ref().unwrap()).replace_child(upper_id.clone(), lower_id.clone());
            } else if self.root.as_ref() == Some(upper_id) {
                self.root = Some(lower_id.clone());
            }

            self.get_mut_unsafe(upper_id).set_parent(Some(lower_id.clone()));
            self.get_mut_unsafe(lower_id).add_child(upper_id.clone());

        } else {

            // just across

            let is_same_parent = self.get_unsafe(first_id).parent() == self.get_unsafe(second_id).parent();

            if is_same_parent {
                let parent_id = self.get_unsafe(first_id).parent().cloned();
                if let Some(parent_id) = parent_id {
                    //same parent
                    //get indices
                    let parent = self.get_mut_unsafe(&parent_id);
                    let first_index = parent.children()
                                    .iter()
                                    .enumerate()
                                    .find(|&(_, id)| id == first_id)
                                    .unwrap().0;
                    let second_index = parent.children()
                                    .iter()
                                    .enumerate()
                                    .find(|&(_, id)| id == second_id)
                                    .unwrap().0;

                    parent.children_mut().swap(first_index, second_index);
                } else {
                    //swapping the root with itself??
                }
            } else {
                let first_parent_id = self.get_unsafe(first_id).parent().cloned().unwrap();
                let second_parent_id = self.get_unsafe(second_id).parent().cloned().unwrap();

                //replace parents
                self.get_mut_unsafe(first_id).set_parent(Some(second_parent_id.clone()));
                self.get_mut_unsafe(second_id).set_parent(Some(first_parent_id.clone()));

                //change children
                self.get_mut_unsafe(&first_parent_id).replace_child(first_id.clone(), second_id.clone());
                self.get_mut_unsafe(&second_parent_id).replace_child(second_id.clone(), first_id.clone());
            }
        }

        Result::Ok(())
    }

    ///
    /// Returns a `Some` value containing the `NodeId` of the root `Node` if it exists.  Otherwise a
    /// `None` value is returned.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(5));
    ///
    /// assert_eq!(&root_id, tree.root_node_id().unwrap());
    /// ```
    ///
    pub fn root_node_id(&self) -> Option<&NodeId> {
        self.root.as_ref()
    }

    // Nothing should make it past this function.
    // If there is a way for a NodeId to be invalid, it should be caught here.
    fn is_valid_node_id(&self, node_id: &NodeId) -> (bool, Option<NodeIdError>) {
        if node_id.tree_id != self.id {
            return (false, Some(NodeIdError::InvalidNodeIdForTree));
        }

        if node_id.index >= self.nodes.len() {
            panic!("NodeId: {:?} is out of bounds. This shouldn't ever happen. This is very likely a bug in id_tree.  Please report this issue.", node_id);
        }

        unsafe {
            if self.nodes.get_unchecked(node_id.index).is_none() {
                return (false, Some(NodeIdError::NodeIdNoLongerValid));
            }
        }

        (true, None)
    }

    fn find_subtree_root_between_ids<'a>(&'a self, lower_id: &'a NodeId, upper_id: &'a NodeId) -> Option<&'a NodeId> {
        if let Some(lower_parent) = self.get_unsafe(lower_id).parent() {
            if lower_parent == upper_id {
                return Some(lower_id);
            } else {
                return self.find_subtree_root_between_ids(lower_parent, upper_id);
            }
        }

        // lower_id has no parent, it can't be below upper_id
        None
    }

    fn set_as_parent_and_child(&mut self, parent_id: &NodeId, child_id: &NodeId) {
        self.get_mut_unsafe(parent_id)
            .add_child(child_id.clone());

        self.get_mut_unsafe(child_id)
            .set_parent(Some(parent_id.clone()));
    }

    fn insert_new_node(&mut self, new_node: Node<T>) -> NodeId {

        if self.free_ids.len() > 0 {
            let new_node_id: NodeId = self.free_ids.pop()
                .expect("Tree::insert_new_node: Couldn't pop from Vec with len() > 0.");

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


        if let Some(root_id) = self.root.clone() {
            if node_id == root_id {
                self.root = None;
            }
        }

        let mut node = self.take_node(node_id.clone());

        //The only thing we care about here is dealing with "this" Node's parent's children
        //This Node's children's parent will be handled in different ways depending upon how this
        //method is called.
        if let Some(parent_id) = node.parent() {
            self.get_mut_unsafe(&parent_id)
                .children_mut()
                .retain(|child_id| child_id != &node_id);
        }

        //avoid providing the caller with extra copies of NodeIds
        node.children_mut().clear();
        node.set_parent(None);

        node
    }

    fn take_node(&mut self, node_id: NodeId) -> Node<T> {
        self.nodes.push(None);
        let node = self.nodes.swap_remove(node_id.index)
            .expect("Tree::take_node: An invalid NodeId made it past id_tree's internal checks.  Please report this issue!");
        self.free_ids.push(node_id);

        node
    }

    fn new_node_id(&self, node_index: usize) -> NodeId {
        NodeId {
            tree_id: self.id,
            index: node_index,
        }
    }

    fn clear_parent(&mut self, node_id: &NodeId) {
        self.get_mut_unsafe(node_id).set_parent(None);
    }

    fn clear_parent_of_children(&mut self, node_id: &NodeId) {
        for child_id in self.get_unsafe(node_id).children().clone() {
            self.clear_parent(&child_id);
        }
    }

    fn get_unsafe(&self, node_id: &NodeId) -> &Node<T> {
        unsafe {
            self.nodes.get_unchecked(node_id.index)
                .as_ref()
                .expect("Tree::get_unsafe: An invalid NodeId made it past id_tree's internal checks.  Please report this issue!")
        }
    }

    fn get_mut_unsafe(&mut self, node_id: &NodeId) -> &mut Node<T> {
        unsafe {
            self.nodes.get_unchecked_mut(node_id.index)
                .as_mut()
                .expect("Tree::get_mut_unsafe: An invalid NodeId made it past id_tree's internal checks.  Please report this issue!")
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

        let root_id = tree.root.clone().unwrap();
        let root = tree.get(&root_id).unwrap();

        assert_eq!(root.data(), &5);
    }

    #[test]
    fn test_get_mut() {
        let mut tree = TreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.root.clone().unwrap();

        {
            let root = tree.get(&root_id).unwrap();
            assert_eq!(root.data(), &5);
        }

        {
            let root = tree.get_mut(&root_id).unwrap();
            *root.data_mut() = 6;
        }

        let root = tree.get(&root_id).unwrap();
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
        let root_id = tree.root.clone().unwrap();
        assert_eq!(node_a_id, root_id);

        {
            let node_a_ref = tree.get(&node_a_id).unwrap();
            let root_ref = tree.get(&root_id).unwrap();
            assert_eq!(node_a_ref.data(), &a);
            assert_eq!(root_ref.data(), &a);
        }

        let node_b_id = tree.set_root(node_b);
        let root_id = tree.root.clone().unwrap();
        assert_eq!(node_b_id, root_id);

        {
            let node_b_ref = tree.get(&node_b_id).unwrap();
            let root_ref = tree.get(&root_id).unwrap();
            assert_eq!(node_b_ref.data(), &b);
            assert_eq!(root_ref.data(), &b);

            let node_b_child_id = node_b_ref.children().get(0).unwrap();
            let node_b_child_ref = tree.get(&node_b_child_id).unwrap();
            assert_eq!(node_b_child_ref.data(), &a);
        }
    }

    #[test]
    fn test_root_node_id() {
        let tree = TreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.root.clone().unwrap();
        let root_node_id = tree.root_node_id().unwrap();

        assert_eq!(&root_id, root_node_id);
    }

    #[test]
    fn test_insert_with_parent() {
        let a = 1;
        let b = 2;
        let r = 5;

        let mut tree = TreeBuilder::new()
            .with_root(Node::new(r))
            .build();

        let node_a = Node::new(a);
        let node_b = Node::new(b);

        let root_id = tree.root.clone().unwrap();
        let node_a_id = tree.insert_with_parent(node_a, &root_id).unwrap();
        let node_b_id = tree.insert_with_parent(node_b, &root_id).unwrap();

        let node_a_ref = tree.get(&node_a_id).unwrap();
        let node_b_ref = tree.get(&node_b_id).unwrap();
        assert_eq!(node_a_ref.data(), &a);
        assert_eq!(node_b_ref.data(), &b);

        assert_eq!(node_a_ref.parent().unwrap().clone(), root_id);
        assert_eq!(node_b_ref.parent().unwrap().clone(), root_id);

        let root_node_ref = tree.get(&root_id).unwrap();
        let root_children: &Vec<NodeId> = root_node_ref.children();

        let child_1_id = root_children.get(0).unwrap();
        let child_2_id = root_children.get(1).unwrap();

        let child_1_ref = tree.get(&child_1_id).unwrap();
        let child_2_ref = tree.get(&child_2_id).unwrap();

        assert_eq!(child_1_ref.data(), &a);
        assert_eq!(child_2_ref.data(), &b);
    }

    #[test]
    fn test_remove_node_lift_children() {

        let mut tree = TreeBuilder::new()
            .with_root(Node::new(5))
            .build();

        let root_id = tree.root.clone().unwrap();

        let node_1_id = tree.insert_with_parent(Node::new(1), &root_id).unwrap();
        let node_2_id = tree.insert_with_parent(Node::new(2), &node_1_id).unwrap();
        let node_3_id = tree.insert_with_parent(Node::new(3), &node_1_id).unwrap();

        let node_1 = tree.remove_node_lift_children(node_1_id.clone()).unwrap();

        assert_eq!(Some(&root_id), tree.root_node_id());

        assert_eq!(node_1.data(), &1);
        assert_eq!(node_1.children().len(), 0);
        assert!(node_1.parent().is_none());
        assert!(tree.get(&node_1_id).is_none());

        let root_ref = tree.get(&root_id).unwrap();
        let node_2_ref = tree.get(&node_2_id).unwrap();
        let node_3_ref = tree.get(&node_3_id).unwrap();

        assert_eq!(node_2_ref.data(), &2);
        assert_eq!(node_3_ref.data(), &3);

        assert_eq!(node_2_ref.parent().unwrap(), &root_id);
        assert_eq!(node_3_ref.parent().unwrap(), &root_id);

        assert!(root_ref.children().contains(&node_2_id));
        assert!(root_ref.children().contains(&node_3_id));
    }

    #[test]
    fn test_remove_node_orphan_children() {

        let mut tree = TreeBuilder::new()
            .with_root(Node::new(5))
            .build();

        let root_id = tree.root.clone().unwrap();

        let node_1_id = tree.insert_with_parent(Node::new(1), &root_id).unwrap();
        let node_2_id = tree.insert_with_parent(Node::new(2), &node_1_id).unwrap();
        let node_3_id = tree.insert_with_parent(Node::new(3), &node_1_id).unwrap();

        let node_1 = tree.remove_node_orphan_children(node_1_id.clone()).unwrap();

        assert_eq!(Some(&root_id), tree.root_node_id());

        assert_eq!(node_1.data(), &1);
        assert_eq!(node_1.children().len(), 0);
        assert!(node_1.parent().is_none());
        assert!(tree.get(&node_1_id).is_none());

        let node_2_ref = tree.get(&node_2_id).unwrap();
        let node_3_ref = tree.get(&node_3_id).unwrap();

        assert_eq!(node_2_ref.data(), &2);
        assert_eq!(node_3_ref.data(), &3);

        assert!(node_2_ref.parent().is_none());
        assert!(node_3_ref.parent().is_none());
    }

    #[test]
    fn test_remove_root() {
        let mut tree = TreeBuilder::new()
            .with_root(Node::new(5))
            .build();

        let root_id = tree.root.clone().unwrap();
        tree.remove_node_orphan_children(root_id.clone()).unwrap();
        assert_eq!(None, tree.root_node_id());

        let mut tree = TreeBuilder::new()
            .with_root(Node::new(5))
            .build();

        let root_id = tree.root.clone().unwrap();
        tree.remove_node_lift_children(root_id.clone()).unwrap();
        assert_eq!(None, tree.root_node_id());
    }

    #[test]
    fn test_move_node_to_parent() {
        let mut tree = Tree::new();

        let root_id = tree.set_root(Node::new(0));
        let node_1_id = tree.insert_with_parent(Node::new(1), &root_id).unwrap();
        let node_2_id = tree.insert_with_parent(Node::new(2), &root_id).unwrap();
        let node_3_id = tree.insert_with_parent(Node::new(3), &node_1_id).unwrap();

        //move 3 "across" the tree
        tree.move_node_to_parent(&node_3_id, &node_2_id).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&node_2_id).unwrap().children().contains(&node_3_id));

        //move 3 "up" the tree
        tree.move_node_to_parent(&node_3_id, &root_id).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_3_id));

        //move 3 "down" (really this is across though) the tree
        tree.move_node_to_parent(&node_3_id, &node_1_id).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&node_1_id).unwrap().children().contains(&node_3_id));

        //move 1 "down" the tree
        tree.move_node_to_parent(&node_1_id, &node_3_id).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_3_id));
        assert!(tree.get(&node_3_id).unwrap().children().contains(&node_1_id));

        //note: node_1 is at the lowest point in the tree before these insertions.
        let node_4_id = tree.insert_with_parent(Node::new(4), &node_1_id).unwrap();
        let node_5_id = tree.insert_with_parent(Node::new(5), &node_4_id).unwrap();

        //move 3 "down" the tree
        tree.move_node_to_parent(&node_3_id, &node_5_id).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&node_1_id).unwrap().children().contains(&node_4_id));
        assert!(tree.get(&node_4_id).unwrap().children().contains(&node_5_id));
        assert!(tree.get(&node_5_id).unwrap().children().contains(&node_3_id));

        //move root "down" the tree
        tree.move_node_to_parent(&root_id, &node_2_id).unwrap();
        assert!(tree.get(&node_2_id).unwrap().children().contains(&root_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&node_1_id).unwrap().children().contains(&node_4_id));
        assert!(tree.get(&node_4_id).unwrap().children().contains(&node_5_id));
        assert!(tree.get(&node_5_id).unwrap().children().contains(&node_3_id));
        assert_eq!(tree.root_node_id(), Some(&node_2_id));

    }

    #[test]
    fn test_find_subtree_root_below_upper_id() {
        let mut tree = Tree::new();

        let root_id = tree.set_root(Node::new(0));
        let node_1_id = tree.insert_with_parent(Node::new(1), &root_id).unwrap();
        let node_2_id = tree.insert_with_parent(Node::new(2), &node_1_id).unwrap();
        let node_3_id = tree.insert_with_parent(Node::new(3), &node_1_id).unwrap();
        let node_4_id = tree.insert_with_parent(Node::new(4), &node_2_id).unwrap();

        let sub_root = tree.find_subtree_root_between_ids(&node_1_id, &root_id);
        assert_eq!(sub_root, Some(&node_1_id));
        let sub_root = tree.find_subtree_root_between_ids(&root_id, &node_1_id); //invert for None
        assert_eq!(sub_root, None);

        let sub_root = tree.find_subtree_root_between_ids(&node_2_id, &root_id);
        assert_eq!(sub_root, Some(&node_1_id));
        let sub_root = tree.find_subtree_root_between_ids(&root_id, &node_2_id); //invert for None
        assert_eq!(sub_root, None);

        let sub_root = tree.find_subtree_root_between_ids(&node_3_id, &node_1_id);
        assert_eq!(sub_root, Some(&node_3_id));
        let sub_root = tree.find_subtree_root_between_ids(&node_1_id, &node_3_id); //invert for None
        assert_eq!(sub_root, None);

        let sub_root = tree.find_subtree_root_between_ids(&node_4_id, &root_id);
        assert_eq!(sub_root, Some(&node_1_id));
        let sub_root = tree.find_subtree_root_between_ids(&root_id, &node_4_id); //invert for None
        assert_eq!(sub_root, None);
    }

    #[test]
    fn test_swap_sub_trees() {

        // test across swap
        {
            let mut tree = Tree::new();
            let root_id = tree.set_root(Node::new(0));
            let node_1_id = tree.insert_with_parent(Node::new(1), &root_id).unwrap();
            let node_2_id = tree.insert_with_parent(Node::new(2), &root_id).unwrap();
            let node_3_id = tree.insert_with_parent(Node::new(3), &node_1_id).unwrap();
            let node_4_id = tree.insert_with_parent(Node::new(4), &node_2_id).unwrap();

            tree.swap_sub_tree(&node_3_id, &node_4_id).unwrap();

            assert!(tree.get(&node_1_id).unwrap().children().contains(&node_4_id));
            assert!(tree.get(&node_2_id).unwrap().children().contains(&node_3_id));
        }

        // test ordering via swap
        {
            let mut tree = Tree::new();
            let root_id = tree.set_root(Node::new(0));
            let node_1_id = tree.insert_with_parent(Node::new(1), &root_id).unwrap();
            let node_2_id = tree.insert_with_parent(Node::new(2), &root_id).unwrap();

            tree.swap_sub_tree(&node_1_id, &node_2_id).unwrap();

            let children = tree.get(&root_id).unwrap().children();
            assert!(children[0] == node_2_id);
            assert!(children[1] == node_1_id);
        }

        // test swap down
        {
            let mut tree = Tree::new();
            let root_id = tree.set_root(Node::new(0));
            let node_1_id = tree.insert_with_parent(Node::new(1), &root_id).unwrap();
            let node_2_id = tree.insert_with_parent(Node::new(2), &root_id).unwrap();
            let node_3_id = tree.insert_with_parent(Node::new(3), &node_1_id).unwrap();

            tree.swap_sub_tree(&root_id, &node_3_id).unwrap();

            assert_eq!(tree.root_node_id(), Some(&node_3_id));

            assert!(tree.get(&node_3_id).unwrap().children().contains(&root_id));

            let children = tree.get(&root_id).unwrap().children();
            assert!(children[0] == node_1_id);
            assert!(children[1] == node_2_id);
        }

        // test swap down without root
        {
            let mut tree = Tree::new();
            let root_id = tree.set_root(Node::new(0));
            let node_1_id = tree.insert_with_parent(Node::new(1), &root_id).unwrap();
            let node_2_id = tree.insert_with_parent(Node::new(2), &root_id).unwrap();
            let node_3_id = tree.insert_with_parent(Node::new(3), &node_1_id).unwrap();

            tree.swap_sub_tree(&node_1_id, &node_3_id).unwrap();

            assert!(tree.get(&node_3_id).unwrap().children().contains(&node_1_id));

            let children = tree.get(&root_id).unwrap().children();
            assert!(children[0] == node_3_id);
            assert!(children[1] == node_2_id);
        }
    }
}
