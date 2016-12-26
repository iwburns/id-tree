///
/// Describes the possible behaviors of the `Tree::remove_node` method.
///
pub enum RemoveBehavior {
    ///
    /// All children will be dropped recursively.  In other words, the entire sub-tree of the `Node`
    /// being removed will be dropped from the tree.  Those `Node`s will no longer exist and
    /// cannot be accessed even if you have the `NodeId` the previously pointed to them.
    ///
    /// This means even without using `Clone` you might end up with copies of invalid `NodeId`s.
    /// Use this behavior with caution.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    /// use id_tree::RemoveBehavior;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(0));
    ///
    /// let child_id = tree.insert_with_parent(Node::new(1), &root_id).ok().unwrap();
    /// let grandchild_id = tree.insert_with_parent(Node::new(2), &child_id).ok().unwrap();
    ///
    /// let child = tree.remove_node(child_id, RemoveBehavior::DropChildren).ok().unwrap();
    ///
    /// assert!(tree.get(&grandchild_id).is_none());
    /// assert_eq!(tree.get(&root_id).unwrap().children().len(), 0);
    /// ```
    ///
    DropChildren,

    ///
    /// If the removed `Node` (let's call it `A`) has a parent, `A`'s parent will become the
    /// parent of `A`'s children.  This effectively just shifts them up one level in the `Tree`.
    ///
    /// If `A` doesn't have a parent, then this behaves exactly like
    /// `RemoveBehavior::OrphanChildren`.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    /// use id_tree::RemoveBehavior;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(0));
    ///
    /// let child_id = tree.insert_with_parent(Node::new(1), &root_id).ok().unwrap();
    /// let grandchild_id = tree.insert_with_parent(Node::new(2), &child_id).ok().unwrap();
    ///
    /// let child = tree.remove_node(child_id, RemoveBehavior::LiftChildren).ok().unwrap();
    ///
    /// assert!(tree.get(&grandchild_id).is_some());
    /// assert!(tree.get(&root_id).unwrap().children().contains(&grandchild_id));
    /// ```
    ///
    LiftChildren,

    ///
    /// All children will have their parent references cleared.  This means nothing will point to
    /// them, but they will still exist in the tree.  Those `Node`s can still be accessed provided
    /// that you have the `NodeId` that points to them.
    ///
    /// ```
    /// use id_tree::Tree;
    /// use id_tree::Node;
    /// use id_tree::RemoveBehavior;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(0));
    ///
    /// let child_id = tree.insert_with_parent(Node::new(1), &root_id).ok().unwrap();
    /// let grandchild_id = tree.insert_with_parent(Node::new(2), &child_id).ok().unwrap();
    ///
    /// let child = tree.remove_node(child_id, RemoveBehavior::OrphanChildren).ok().unwrap();
    ///
    /// assert!(tree.get(&grandchild_id).is_some());
    /// assert_eq!(tree.get(&root_id).unwrap().children().len(), 0);
    /// ```
    ///
    OrphanChildren,
}

