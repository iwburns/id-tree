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
    DropChildren,

    ///
    /// If the removed `Node` (let's call it `A`) has a parent, `A`'s parent will become the
    /// parent of `A`'s children.  This effectively just shifts them up one level in the `Tree`.
    ///
    /// If `A` doesn't have a parent, then this behaves exactly like
    /// `RemoveBehavior::OrphanChildren`.
    ///
    LiftChildren,

    ///
    /// All children will have their parent references cleared.  This means nothing will point to
    /// them, but they will still exist in the tree.  Those `Node`s can still be accessed provided
    /// that you have the `NodeId` that points to them.
    ///
    OrphanChildren,
}

