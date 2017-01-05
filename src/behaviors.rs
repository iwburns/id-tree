use super::NodeId;

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
    /// use id_tree::*;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(0));
    ///
    /// let child_id = tree.insert_with_parent(Node::new(1), &root_id).ok().unwrap();
    /// let grandchild_id = tree.insert_with_parent(Node::new(2), &child_id).ok().unwrap();
    ///
    /// let child = tree.remove_node(child_id, RemoveBehavior::DropChildren).ok().unwrap();
    ///
    /// assert!(tree.get(&grandchild_id).is_err());
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
    /// use id_tree::*;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(0));
    ///
    /// let child_id = tree.insert_with_parent(Node::new(1), &root_id).ok().unwrap();
    /// let grandchild_id = tree.insert_with_parent(Node::new(2), &child_id).ok().unwrap();
    ///
    /// let child = tree.remove_node(child_id, RemoveBehavior::LiftChildren).ok().unwrap();
    ///
    /// assert!(tree.get(&grandchild_id).is_ok());
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
    /// use id_tree::*;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(0));
    ///
    /// let child_id = tree.insert_with_parent(Node::new(1), &root_id).ok().unwrap();
    /// let grandchild_id = tree.insert_with_parent(Node::new(2), &child_id).ok().unwrap();
    ///
    /// let child = tree.remove_node(child_id, RemoveBehavior::OrphanChildren).ok().unwrap();
    ///
    /// assert!(tree.get(&grandchild_id).is_ok());
    /// assert_eq!(tree.get(&root_id).unwrap().children().len(), 0);
    /// ```
    ///
    OrphanChildren,
}

///
/// Describes the possible behaviors of the `Tree::move_node` method.
///
pub enum MoveBehavior<'a> {
    ///
    /// Sets the `Node` in question as the new root `Node`, leaving all children in their place (in
    /// other words, they will travel with the `Node` being moved).
    ///
    /// If there is already a root `Node` in place, it will be attached as the last child of the new
    /// root.
    ///
    /// ```
    /// use id_tree::*;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(1));
    ///
    /// let child_id = tree.insert_with_parent(Node::new(2),  &root_id).ok().unwrap();
    /// let grandchild_id = tree.insert_with_parent(Node::new(3), &child_id).ok().unwrap();
    ///
    /// tree.move_node(&grandchild_id, MoveBehavior::ToRoot).unwrap();
    ///
    /// assert_eq!(tree.root_node_id(), Some(&grandchild_id));
    /// assert!(tree.get(&grandchild_id).unwrap().children().contains(&root_id));
    /// assert!(!tree.get(&child_id).unwrap().children().contains(&grandchild_id));
    /// ```
    ///
    ToRoot,

    ///
    /// Moves a `Node` inside the `Tree` to a new parent leaving all children in their place.
    ///
    /// If the new parent (let's call it `B`) is a descendant of the `Node` being moved (`A`), then
    /// the direct child of `A` on the path from `A` to `B` will be shifted upwards to take the
    /// place of its parent (`A`).  All other children of `A` will be left alone, meaning they will
    /// travel with it down the `Tree`.
    ///
    /// Please note that during the "shift-up" part of the above scenario, the `Node` being shifted
    /// up will always be added as the last child of its new parent.
    ///
    /// ```
    /// use id_tree::*;
    ///
    /// let mut tree: Tree<i32> = Tree::new();
    /// let root_id = tree.set_root(Node::new(1));
    ///
    /// let first_child_id = tree.insert_with_parent(Node::new(2),  &root_id).ok().unwrap();
    /// let second_child_id = tree.insert_with_parent(Node::new(3), &root_id).ok().unwrap();
    /// let grandchild_id = tree.insert_with_parent(Node::new(4), &first_child_id).ok().unwrap();
    ///
    /// tree.move_node(&grandchild_id, MoveBehavior::ToParent(&second_child_id)).unwrap();
    ///
    /// assert!(!tree.get(&first_child_id).unwrap().children().contains(&grandchild_id));
    /// assert!(tree.get(&second_child_id).unwrap().children().contains(&grandchild_id));
    /// ```
    ///
    ToParent(&'a NodeId),
}

///
/// Describes the possible behaviors of the `Tree::insert` method.
///
pub enum InsertBehavior<'a> {
    AsRoot,
    UnderNode(&'a NodeId),
}
