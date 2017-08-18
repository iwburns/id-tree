mod vec_tree;
mod opt_tree;

pub use self::vec_tree::*;
pub use self::opt_tree::*;

use std::cmp::Ordering;
use behaviors::*;
use NodeId;
use NodeIdError;

use node::Node;

pub trait Tree<'a, Data> {
    type NodeType: Node<Data>;
    type AncestorsIter: Iterator;
    type AncestorIdsIter: Iterator;
    type ChildrenIter: Iterator;
    type ChildrenIdsIter: Iterator;
    type PreOrderIter: Iterator;
    type PostOrderIter: Iterator;
    type LevelOrderIter: Iterator;

    ///
    /// Creates a new `Tree` with default settings (no root `Node` and no space
    /// pre-allocation).
    ///
    /// ```
    /// use id_tree::VecTree;
    /// use id_tree::Tree;
    ///
    /// let _tree: VecTree<i32> = VecTree::new();
    /// ```
    ///
    fn new() -> Self;

    ///
    /// Inserts a new `Node` into the `Tree`.  The `InsertBehavior` provided will determine
    /// where the `Node` is inserted.
    ///
    /// Returns a `Result` containing the `NodeId` of the `Node` that was inserted or a
    /// `NodeIdError` if one occurred.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let root_node = VecNode::new(1);
    /// let child_node = VecNode::new(2);
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(root_node, AsRoot).unwrap();
    ///
    /// tree.insert(child_node, UnderNode(&root_id)).unwrap();
    /// ```
    ///
    fn insert(
        &mut self,
        node: Self::NodeType,
        behavior: InsertBehavior,
    ) -> Result<NodeId, NodeIdError>;

    ///
    /// Get an immutable reference to a `Node`.
    ///
    /// Returns a `Result` containing the immutable reference or a `NodeIdError` if one occurred.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(5), AsRoot).unwrap();
    ///
    /// let root_node: &Node<i32> = tree.get(&root_id).unwrap();
    ///
    /// # assert_eq!(root_node.data(), &5);
    /// ```
    ///
    fn get(&self, node_id: &NodeId) -> Result<&Self::NodeType, NodeIdError>;

    ///
    /// Get a mutable reference to a `Node`.
    ///
    /// Returns a `Result` containing the mutable reference or a `NodeIdError` if one occurred.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(5), AsRoot).unwrap();
    ///
    /// let root_node: &mut VecNode<i32> = tree.get_mut(&root_id).unwrap();
    ///
    /// # assert_eq!(root_node.data(), &5);
    /// ```
    ///
    fn get_mut(&mut self, node_id: &NodeId) -> Result<&mut Self::NodeType, NodeIdError>;

    /// Remove a `Node` from the `Tree`.  The `RemoveBehavior` provided determines what
    /// happens to the removed `Node`'s children.
    ///
    /// Returns a `Result` containing the removed `Node` or a `NodeIdError` if one occurred.
    ///
    /// **NOTE:** The `Node` that is returned will have its parent and child values cleared to
    /// avoid providing the caller with extra copies of `NodeId`s should the corresponding
    /// `Node`s be removed from the `Tree` at a later time.
    ///
    /// If the caller needs a copy of the parent or child `NodeId`s, they must `Clone` them before
    /// this `Node` is removed from the `Tree`.  Please see the
    /// [Potential `NodeId` Issues](struct.NodeId.html#potential-nodeid-issues) section
    /// of the `NodeId` documentation for more information on the implications of calling `Clone`
    /// on a `NodeId`.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    /// use id_tree::RemoveBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(0), AsRoot).unwrap();
    ///
    /// let child_id = tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    /// let grandchild_id = tree.insert(VecNode::new(2), UnderNode(&child_id)).unwrap();
    ///
    /// let child = tree.remove(child_id, DropChildren).unwrap();
    ///
    /// # assert!(tree.get(&grandchild_id).is_err());
    /// # assert_eq!(tree.get(&root_id).unwrap().children().len(), 0);
    /// # assert_eq!(child.children().len(), 0);
    /// # assert_eq!(child.parent(), None);
    /// ```
    ///
    fn remove(
        &mut self,
        node_id: NodeId,
        behavior: RemoveBehavior,
    ) -> Result<Self::NodeType, NodeIdError>;

    ///
    /// Moves a `Node` in the `Tree` to a new location based upon the `MoveBehavior`
    /// provided.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    /// use id_tree::MoveBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    ///
    /// let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    /// let child_id = tree.insert(VecNode::new(2),  UnderNode(&root_id)).unwrap();
    /// let grandchild_id = tree.insert(Node::new(3), UnderNode(&child_id)).unwrap();
    ///
    /// tree.move_node(&grandchild_id, ToRoot).unwrap();
    ///
    /// assert_eq!(tree.root_node_id(), Some(&grandchild_id));
    /// # assert!(tree.get(&grandchild_id).unwrap().children().contains(&root_id));
    /// # assert!(!tree.get(&child_id).unwrap().children().contains(&grandchild_id));
    /// ```
    ///
    fn move_node(&mut self, node_id: &NodeId, behavior: MoveBehavior) -> Result<(), NodeIdError>;

    ///
    /// Sorts the children of one node, in-place, using compare to compare the nodes
    ///
    /// This sort is stable and O(n log n) worst-case but allocates approximately 2 * n where n is
    /// the length of children
    ///
    /// Returns an empty `Result` containing a `NodeIdError` if one occurred.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    ///
    /// let root_id = tree.insert(VecNode::new(100), AsRoot).unwrap();
    /// tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    /// tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    /// tree.insert(VecNode::new(0), UnderNode(&root_id)).unwrap();
    ///
    /// tree.sort_children_by(&root_id, |a, b| a.data().cmp(b.data())).unwrap();
    ///
    /// # for (i, id) in tree.get(&root_id).unwrap().children().iter().enumerate() {
    /// #   assert_eq!(*tree.get(&id).unwrap().data(), i as i32);
    /// # }
    /// ```
    ///
    fn sort_children_by<F>(&mut self, node_id: &NodeId, compare: F) -> Result<(), NodeIdError>
    where
        F: FnMut(&Self::NodeType, &Self::NodeType) -> Ordering;

    ///
    /// Sorts the children of one node, in-place, comparing their data
    ///
    /// This sort is stable and O(n log n) worst-case but allocates approximately 2 * n where n is
    /// the length of children
    ///
    /// Returns an empty `Result` containing a `NodeIdError` if one occurred.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    ///
    /// let root_id = tree.insert(VecNode::new(100), AsRoot).unwrap();
    /// tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    /// tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    /// tree.insert(VecNode::new(0), UnderNode(&root_id)).unwrap();
    ///
    /// tree.sort_children_by_data(&root_id).unwrap();
    ///
    /// # for (i, id) in tree.get(&root_id).unwrap().children().iter().enumerate() {
    /// #   assert_eq!(*tree.get(&id).unwrap().data(), i as i32);
    /// # }
    /// ```
    ///
    fn sort_children_by_data(&mut self, node_id: &NodeId) -> Result<(), NodeIdError>
    where
        Data: Ord;

    ///
    /// Sorts the children of one node, in-place, using f to extract a key by which to order the
    /// sort by.
    ///
    /// This sort is stable and O(n log n) worst-case but allocates approximately 2 * n where n is
    /// the length of children
    ///
    /// Returns an empty `Result` containing a `NodeIdError` if one occurred.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    ///
    /// let root_id = tree.insert(VecNode::new(100), AsRoot).unwrap();
    /// tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    /// tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    /// tree.insert(VecNode::new(0), UnderNode(&root_id)).unwrap();
    ///
    /// tree.sort_children_by_key(&root_id, |x| x.data().clone()).unwrap();
    ///
    /// # for (i, id) in tree.get(&root_id).unwrap().children().iter().enumerate() {
    /// #   assert_eq!(*tree.get(&id).unwrap().data(), i as i32);
    /// # }
    /// ```
    ///
    fn sort_children_by_key<K, F>(&mut self, node_id: &NodeId, f: F) -> Result<(), NodeIdError>
    where
        K: Ord,
        F: FnMut(&Self::NodeType) -> K;

    /// Swap `Node`s in the `Tree` based upon the `SwapBehavior` provided.
    ///
    /// Both `NodeId`s are still valid after this process and are not swapped.
    ///
    /// This keeps the positions of the `Node`s in their parents' children collection.
    ///
    /// Returns an empty `Result` containing a `NodeIdError` if one occurred on either provided
    /// `NodeId`.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    /// use id_tree::SwapBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    ///
    /// let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    ///
    /// let first_child_id = tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    /// let second_child_id = tree.insert(VecNode::new(3), UnderNode(&root_id)).unwrap();
    /// let grandchild_id = tree.insert(VecNode::new(4), UnderNode(&second_child_id)).unwrap();
    ///
    /// tree.swap_nodes(&first_child_id, &grandchild_id, TakeChildren).unwrap();
    ///
    /// assert!(tree.get(&second_child_id).unwrap().children().contains(&first_child_id));
    /// assert!(tree.get(&root_id).unwrap().children().contains(&grandchild_id));
    /// ```
    ///
    fn swap_nodes(
        &mut self,
        first_id: &NodeId,
        second_id: &NodeId,
        behavior: SwapBehavior,
    ) -> Result<(), NodeIdError>;

    ///
    /// Returns a `Some` value containing the `NodeId` of the root `Node` if it exists.
    /// Otherwise a `None` value is returned.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(5), AsRoot).unwrap();
    ///
    /// assert_eq!(&root_id, tree.root_node_id().unwrap());
    /// ```
    ///
    fn root_node_id(&self) -> Option<&NodeId>;

    ///
    /// Returns an `Ancestors` iterator (or a `NodeIdError` if one occurred).
    ///
    /// Allows iteration over the ancestor `Node`s of a given `NodeId` directly instead of
    /// having to call `tree.get(...)` with a `NodeId` each time.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(0), AsRoot).unwrap();
    /// let node_1 = tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    ///
    /// let mut ancestors = tree.ancestors(&node_1).unwrap();
    ///
    /// assert_eq!(ancestors.next().unwrap().data(), &0);
    /// assert!(ancestors.next().is_none());
    /// ```
    ///
    fn ancestors(&'a self, node_id: &NodeId) -> Result<Self::AncestorsIter, NodeIdError>;

    ///
    /// Returns an `AncestorIds` iterator (or a `NodeIdError` if one occurred).
    ///
    /// Allows iteration over the ancestor `NodeId`s of a given `NodeId`.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(0), AsRoot).unwrap();
    /// let node_1 = tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    ///
    /// let mut ancestor_ids = tree.ancestor_ids(&node_1).unwrap();
    ///
    /// assert_eq!(ancestor_ids.next().unwrap(), &root_id);
    /// assert!(ancestor_ids.next().is_none());
    /// ```
    ///
    fn ancestor_ids(&'a self, node_id: &NodeId) -> Result<Self::AncestorIdsIter, NodeIdError>;

    ///
    /// Returns a `Children` iterator (or a `NodeIdError` if one occurred).
    ///
    /// Allows iteration over the child `Node`s of a given `NodeId` directly instead of having
    /// to call `tree.get(...)` with a `NodeId` each time.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(0), AsRoot).unwrap();
    /// tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    ///
    /// let mut children = tree.children(&root_id).unwrap();
    ///
    /// assert_eq!(children.next().unwrap().data(), &1);
    /// assert!(children.next().is_none());
    /// ```
    ///
    fn children(&'a self, node_id: &NodeId) -> Result<Self::ChildrenIter, NodeIdError>;


    ///
    /// Returns a `ChildrenIds` iterator (or a `NodeIdError` if one occurred).
    ///
    /// Allows iteration over the child `NodeId`s of a given `NodeId`.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(0), AsRoot).unwrap();
    /// let node_1 = tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    ///
    /// let mut children_ids = tree.children_ids(&root_id).unwrap();
    ///
    /// assert_eq!(children_ids.next().unwrap(), &node_1);
    /// assert!(children_ids.next().is_none());
    /// ```
    ///
    fn children_ids(&'a self, node_id: &NodeId) -> Result<Self::ChildrenIdsIter, NodeIdError>;

    ///
    /// Returns a `PreOrderTraversal` iterator (or a `NodeIdError` if one occurred).
    ///
    /// Allows iteration over all of the `Node`s in the sub-tree below a given `Node`.  This
    /// iterator will always include that sub-tree "root" specified by the `NodeId` given.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(0), AsRoot).unwrap();
    /// tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    ///
    /// let mut nodes = tree.traverse_pre_order(&root_id).unwrap();
    ///
    /// assert_eq!(nodes.next().unwrap().data(), &0);
    /// assert_eq!(nodes.next().unwrap().data(), &1);
    /// assert!(nodes.next().is_none());
    /// ```
    ///
    fn traverse_pre_order(&'a self, node_id: &NodeId) -> Result<Self::PreOrderIter, NodeIdError>;

    ///
    /// Returns a `PostOrderTraversal` iterator (or a `NodeIdError` if one occurred).
    ///
    /// Allows iteration over all of the `Node`s in the sub-tree below a given `Node`.  This
    /// iterator will always include that sub-tree "root" specified by the `NodeId` given.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(0), AsRoot).unwrap();
    /// tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    ///
    /// let mut nodes = tree.traverse_post_order(&root_id).unwrap();
    ///
    /// assert_eq!(nodes.next().unwrap().data(), &1);
    /// assert_eq!(nodes.next().unwrap().data(), &0);
    /// assert!(nodes.next().is_none());
    /// ```
    ///
    fn traverse_post_order(&'a self, node_id: &NodeId) -> Result<Self::PostOrderIter, NodeIdError>;

    ///
    /// Returns a `LevelOrderTraversal` iterator (or a `NodeIdError` if one occurred).
    ///
    /// Allows iteration over all of the `Node`s in the sub-tree below a given `Node`.  This
    /// iterator will always include that sub-tree "root" specified by the `NodeId` given.
    ///
    /// ```
    /// use id_tree::*;
    /// use id_tree::InsertBehavior::*;
    ///
    /// let mut tree: VecTree<i32> = VecTree::new();
    /// let root_id = tree.insert(VecNode::new(0), AsRoot).unwrap();
    /// tree.insert(VecNode::new(1), UnderNode(&root_id)).unwrap();
    ///
    /// let mut nodes = tree.traverse_level_order(&root_id).unwrap();
    ///
    /// assert_eq!(nodes.next().unwrap().data(), &0);
    /// assert_eq!(nodes.next().unwrap().data(), &1);
    /// assert!(nodes.next().is_none());
    /// ```
    ///
    fn traverse_level_order(
        &'a self,
        node_id: &NodeId,
    ) -> Result<Self::LevelOrderIter, NodeIdError>;
}
