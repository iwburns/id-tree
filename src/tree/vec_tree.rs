use std::cmp::Ordering;
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

///
/// A tree structure consisting of `VecNode`s.
///
/// # Panics
/// While it is highly unlikely, any function that takes a `NodeId` _can_ `panic`.  This, however,
/// should only happen due to improper `NodeId` management within `id_tree` and should have nothing
/// to do with the library user's code.
///
/// **If this ever happens please report the issue.** `Panic`s are not expected behavior for this
/// library, but they can happen due to bugs.
///
pub struct VecTree<'a, T: 'a> {
    core_tree: CoreTree<VecNode<T>, T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Tree<'a, T> for VecTree<'a, T> {
    type NodeType = VecNode<T>;
    type AncestorsIter = Ancestors<'a, VecTree<'a, T>, T>;
    type AncestorIdsIter = AncestorIds<'a, VecTree<'a, T>, T>;
    type ChildrenIter = VecChildren<'a, T>;
    type ChildrenIdsIter = VecChildrenIds<'a>;
    type PreOrderIter = VecPreOrderTraversal<'a, T>;
    type PostOrderIter = VecPostOrderTraversal<'a, T>;
    type LevelOrderIter = VecLevelOrderTraversal<'a, T>;

    fn new() -> VecTree<'a, T> {
        VecTreeBuilder::new().build()
    }

    fn insert(
        &mut self,
        node: VecNode<T>,
        behavior: InsertBehavior,
    ) -> Result<NodeId, NodeIdError> {
        match behavior {
            InsertBehavior::UnderNode(parent_id) => {
                self.core_tree.validate_node_id(parent_id)?;
                self.insert_with_parent(node, parent_id)
            }
            InsertBehavior::AsRoot => Ok(self.set_root(node)),
        }
    }

    fn get(&self, node_id: &NodeId) -> Result<&VecNode<T>, NodeIdError> {
        self.core_tree.get(node_id)
    }

    fn get_mut(&mut self, node_id: &NodeId) -> Result<&mut VecNode<T>, NodeIdError> {
        self.core_tree.get_mut(node_id)
    }

    fn remove(
        &mut self,
        node_id: NodeId,
        behavior: RemoveBehavior,
    ) -> Result<VecNode<T>, NodeIdError> {
        self.core_tree.validate_node_id(&node_id)?;
        match behavior {
            RemoveBehavior::DropChildren => self.remove_node_drop_children(node_id),
            RemoveBehavior::LiftChildren => self.remove_node_lift_children(node_id),
            RemoveBehavior::OrphanChildren => self.remove_node_orphan_children(node_id),
        }
    }

    fn move_node(&mut self, node_id: &NodeId, behavior: MoveBehavior) -> Result<(), NodeIdError> {
        self.core_tree.validate_node_id(node_id)?;
        match behavior {
            MoveBehavior::ToRoot => self.move_node_to_root(node_id),
            MoveBehavior::ToParent(parent_id) => {
                self.core_tree.validate_node_id(parent_id)?;
                self.move_node_to_parent(node_id, parent_id)
            }
        }
    }

    fn sort_children_by<F>(&mut self, node_id: &NodeId, mut compare: F) -> Result<(), NodeIdError>
    where
        F: FnMut(&VecNode<T>, &VecNode<T>) -> Ordering,
    {
        self.core_tree.validate_node_id(node_id)?;

        let mut children = self.core_tree.get_mut_unsafe(node_id).take_children();
        children.sort_by(|a, b| {
            compare(self.core_tree.get_unsafe(a), self.core_tree.get_unsafe(b))
        });
        self.core_tree.get_mut_unsafe(node_id).set_children(
            children,
        );

        Ok(())
    }

    fn sort_children_by_data(&mut self, node_id: &NodeId) -> Result<(), NodeIdError>
    where
        T: Ord,
    {
        self.core_tree.validate_node_id(node_id)?;

        let mut children = self.core_tree.get_mut_unsafe(node_id).take_children();
        children.sort_by_key(|a| self.core_tree.get_unsafe(a).data());
        self.core_tree.get_mut_unsafe(node_id).set_children(
            children,
        );

        Ok(())
    }

    fn sort_children_by_key<B, F>(&mut self, node_id: &NodeId, mut f: F) -> Result<(), NodeIdError>
    where
        B: Ord,
        F: FnMut(&VecNode<T>) -> B,
    {
        self.core_tree.validate_node_id(node_id)?;

        let mut children = self.core_tree.get_mut_unsafe(node_id).take_children();
        children.sort_by_key(|a| f(self.core_tree.get_unsafe(a)));
        self.core_tree.get_mut_unsafe(node_id).set_children(
            children,
        );

        Result::Ok(())
    }

    fn swap_nodes(
        &mut self,
        first_id: &NodeId,
        second_id: &NodeId,
        behavior: SwapBehavior,
    ) -> Result<(), NodeIdError> {

        self.core_tree.validate_node_id(first_id)?;
        self.core_tree.validate_node_id(second_id)?;

        match behavior {
            SwapBehavior::TakeChildren => self.swap_nodes_take_children(first_id, second_id),
            SwapBehavior::LeaveChildren => self.swap_nodes_leave_children(first_id, second_id),
            SwapBehavior::ChildrenOnly => self.swap_nodes_children_only(first_id, second_id),
        }
    }

    fn root_node_id(&self) -> Option<&NodeId> {
        self.core_tree.root()
    }

    fn ancestors(&'a self, node_id: &NodeId) -> Result<Self::AncestorsIter, NodeIdError> {
        let (is_valid, error) = self.core_tree.is_valid_node_id(node_id);
        if !is_valid {
            return Err(error.expect(
                "VecTree::ancestors: Missing an error value but found an invalid NodeId.",
            ));
        }

        Ok(Ancestors::new(self, node_id.clone()))
    }

    fn ancestor_ids(&'a self, node_id: &NodeId) -> Result<Self::AncestorIdsIter, NodeIdError> {
        let (is_valid, error) = self.core_tree.is_valid_node_id(node_id);
        if !is_valid {
            return Err(error.expect(
                "VecTree::ancestor_ids: Missing an error value but found an invalid NodeId.",
            ));
        }

        Ok(AncestorIds::new(self, node_id.clone()))
    }

    fn children(&'a self, node_id: &NodeId) -> Result<Self::ChildrenIter, NodeIdError> {
        let (is_valid, error) = self.core_tree.is_valid_node_id(node_id);
        if !is_valid {
            return Err(error.expect(
                "VecTree::children: Missing an error value but found an invalid NodeId.",
            ));
        }

        Ok(VecChildren::new(self, node_id.clone()))
    }

    fn children_ids(&'a self, node_id: &NodeId) -> Result<Self::ChildrenIdsIter, NodeIdError> {
        let (is_valid, error) = self.core_tree.is_valid_node_id(node_id);
        if !is_valid {
            return Err(error.expect(
                "VecTree::children_ids: Missing an error value but found an invalid NodeId.",
            ));
        }

        Ok(VecChildrenIds::new(self, node_id.clone()))
    }

    fn traverse_pre_order(&'a self, node_id: &NodeId) -> Result<Self::PreOrderIter, NodeIdError> {
        let (is_valid, error) = self.core_tree.is_valid_node_id(node_id);
        if !is_valid {
            return Err(error.expect(
                "VecTree::traverse_pre_order: Missing an error value but found an invalid NodeId.",
            ));
        }

        Ok(VecPreOrderTraversal::new(self, node_id.clone()))
    }

    fn traverse_post_order(&'a self, node_id: &NodeId) -> Result<Self::PostOrderIter, NodeIdError> {
        let (is_valid, error) = self.core_tree.is_valid_node_id(node_id);
        if !is_valid {
            return Err(error.expect(
                "VecTree::traverse_post_order: Missing an error value but found an invalid\
                NodeId.",
            ));
        }

        Ok(VecPostOrderTraversal::new(self, node_id.clone()))
    }

    fn traverse_level_order(
        &'a self,
        node_id: &NodeId,
    ) -> Result<Self::LevelOrderIter, NodeIdError> {
        let (is_valid, error) = self.core_tree.is_valid_node_id(node_id);
        if !is_valid {
            return Err(error.expect(
                "VecTree::traverse_level_order: Missing an error value but found an invalid\
                NodeId.",
            ));
        }

        Ok(VecLevelOrderTraversal::new(self, node_id.clone()))
    }
}

impl<'a, T> VecTree<'a, T> {
    ///
    /// Sets the root of the `Tree`.
    ///
    fn set_root(&mut self, new_root: VecNode<T>) -> NodeId {

        let current_root = self.core_tree.root.clone();
        let new_root_id = self.core_tree.set_root(new_root);

        if let Some(current_root_node_id) = current_root {
            self.set_as_parent_and_child(&new_root_id, &current_root_node_id);
        }

        new_root_id
    }

    ///
    /// Add a new `NodeVec` to the tree as the child of a `NodeVec` specified by the given
    /// `NodeId`.
    ///
    fn insert_with_parent(
        &mut self,
        child: VecNode<T>,
        parent_id: &NodeId,
    ) -> Result<NodeId, NodeIdError> {
        let new_child_id = self.core_tree.insert(child);
        self.set_as_parent_and_child(parent_id, &new_child_id);
        Ok(new_child_id)
    }

    ///
    /// Remove a `VecNode` from the `VecTree` and move its children up one "level" in the `Tree` if
    /// possible.
    ///
    /// In other words, this `VecNode`'s children will point to its parent as their parent instead
    /// of this `VecNode`.  In addition, this `VecNode`'s parent will have this `VecNode`'s
    /// children added as its own children.  If this `VecNode` has no parent, then calling this
    /// function is the equivalent of calling `remove_node_orphan_children`.
    ///
    fn remove_node_lift_children(&mut self, node_id: NodeId) -> Result<VecNode<T>, NodeIdError> {
        if let Some(parent_id) = self.core_tree.get_unsafe(&node_id).parent().cloned() {
            // attach children to parent
            for child_id in self.core_tree.get_unsafe(&node_id).children().clone() {
                self.set_as_parent_and_child(&parent_id, &child_id);
            }
        } else {
            self.clear_parent_of_children(&node_id);
        }

        Ok(self.remove_node_internal(node_id))
    }

    ///
    /// Remove a `VecNode` from the `VecTree` and leave all of its children in the `Tree`.
    ///
    fn remove_node_orphan_children(&mut self, node_id: NodeId) -> Result<VecNode<T>, NodeIdError> {
        self.clear_parent_of_children(&node_id);
        Ok(self.remove_node_internal(node_id))
    }

    ///
    /// Remove a `VecNode` from the `VecTree` including all its children recursively.
    ///
    fn remove_node_drop_children(&mut self, node_id: NodeId) -> Result<VecNode<T>, NodeIdError> {
        let mut children = self.core_tree.get_mut_unsafe(&node_id).take_children();
        for child in children.drain(..) {
            self.remove_node_drop_children(child)?;
        }
        Ok(self.remove_node_internal(node_id))
    }

    ///
    /// Moves a `VecNode` inside a `VecTree` to a new parent leaving all children in their place.
    ///
    fn move_node_to_parent(
        &mut self,
        node_id: &NodeId,
        parent_id: &NodeId,
    ) -> Result<(), NodeIdError> {
        if let Some(subtree_root_id) =
            self.find_subtree_root_between_ids(parent_id, node_id)
                .cloned()
        {
            // node_id is above parent_id, this is a move "down" the tree.

            let root = self.core_tree.root.clone();

            if root.as_ref() == Some(node_id) {
                // we're moving the root down the tree.
                // also we know the root exists

                // detach subtree_root from node
                self.detach_from_parent(node_id, &subtree_root_id);

                // set subtree_root as Tree root.
                self.clear_parent(&subtree_root_id);
                self.core_tree.root = Some(subtree_root_id);

                self.set_as_parent_and_child(parent_id, node_id);

            } else {
                // we're moving some other node down the tree.

                if let Some(old_parent) = self.core_tree.get_unsafe(node_id).parent().cloned() {
                    // detach from old parent
                    self.detach_from_parent(&old_parent, node_id);
                    // connect old parent and subtree root
                    self.set_as_parent_and_child(&old_parent, &subtree_root_id);
                } else {
                    // node is orphaned, need to set subtree_root's parent to None (same as node's)
                    self.clear_parent(&subtree_root_id);
                }
                // detach subtree_root from node
                self.detach_from_parent(node_id, &subtree_root_id);

                self.set_as_parent_and_child(parent_id, node_id);
            }

        } else {
            // this is a move "across" or "up" the tree.

            // detach from old parent
            if let Some(old_parent) = self.core_tree.get_unsafe(node_id).parent().cloned() {
                self.detach_from_parent(&old_parent, node_id);
            }

            self.set_as_parent_and_child(parent_id, node_id);
        }

        Ok(())
    }

    ///
    /// Sets a `VecNode` inside a `VecTree` as the new root `VecNode`, leaving all children in
    /// their place.
    ///
    fn move_node_to_root(&mut self, node_id: &NodeId) -> Result<(), NodeIdError> {
        let old_root = self.core_tree.root.clone();

        if let Some(parent_id) = self.core_tree.get_unsafe(node_id).parent().cloned() {
            self.detach_from_parent(&parent_id, node_id);
        }
        self.clear_parent(node_id);
        self.core_tree.root = Some(node_id.clone());

        if let Some(old_root) = old_root {
            self.move_node_to_parent(&old_root, node_id)?;
        }

        Ok(())
    }

    ///
    /// Swaps two `VecNode`s including their children given their `NodeId`s.
    ///
    fn swap_nodes_take_children(
        &mut self,
        first_id: &NodeId,
        second_id: &NodeId,
    ) -> Result<(), NodeIdError> {
        let lower_upper_test = self.find_subtree_root_between_ids(first_id, second_id)
            .map(|_| (first_id, second_id))
            .or_else(|| {
                self.find_subtree_root_between_ids(second_id, first_id)
                    .map(|_| (second_id, first_id))
            });

        if let Some((lower_id, upper_id)) = lower_upper_test {
            let upper_parent_id = self.core_tree.get_unsafe(upper_id).parent().cloned();

            let lower_parent_id = {
                let lower = self.core_tree.get_mut_unsafe(lower_id);
                // lower is lower, so it has a parent for sure
                let lower_parent_id = lower.parent().unwrap().clone();

                if upper_parent_id.is_some() {
                    lower.set_parent(upper_parent_id.clone());
                } else {
                    lower.set_parent(None);
                }

                lower_parent_id
            };

            self.detach_from_parent(&lower_parent_id, lower_id);

            if upper_parent_id.is_some() {
                self.core_tree
                    .get_mut_unsafe(upper_parent_id.as_ref().unwrap())
                    .replace_child(upper_id.clone(), lower_id.clone());
            } else if self.core_tree.root.as_ref() == Some(upper_id) {
                self.core_tree.root = Some(lower_id.clone());
            }

            self.core_tree.get_mut_unsafe(upper_id).set_parent(Some(
                lower_id.clone(),
            ));
            self.core_tree.get_mut_unsafe(lower_id).add_child(
                upper_id.clone(),
            );

        } else {

            // just across

            let is_same_parent = self.core_tree.get_unsafe(first_id).parent() ==
                self.core_tree.get_unsafe(second_id).parent();

            if is_same_parent {
                let parent_id = self.core_tree.get_unsafe(first_id).parent().cloned();
                if let Some(parent_id) = parent_id {
                    // same parent
                    // get indices
                    let parent = self.core_tree.get_mut_unsafe(&parent_id);
                    let first_index = parent
                        .children()
                        .iter()
                        .enumerate()
                        .find(|&(_, id)| id == first_id)
                        .unwrap()
                        .0;
                    let second_index = parent
                        .children()
                        .iter()
                        .enumerate()
                        .find(|&(_, id)| id == second_id)
                        .unwrap()
                        .0;

                    parent.children_mut().swap(first_index, second_index);
                } else {
                    // swapping the root with itself??
                }
            } else {
                let first_parent_id = self.core_tree
                    .get_unsafe(first_id)
                    .parent()
                    .cloned()
                    .unwrap();
                let second_parent_id = self.core_tree
                    .get_unsafe(second_id)
                    .parent()
                    .cloned()
                    .unwrap();

                // replace parents
                self.core_tree.get_mut_unsafe(first_id).set_parent(Some(
                    second_parent_id
                        .clone(),
                ));
                self.core_tree.get_mut_unsafe(second_id).set_parent(Some(
                    first_parent_id
                        .clone(),
                ));

                // change children
                self.core_tree
                    .get_mut_unsafe(&first_parent_id)
                    .replace_child(first_id.clone(), second_id.clone());
                self.core_tree
                    .get_mut_unsafe(&second_parent_id)
                    .replace_child(second_id.clone(), first_id.clone());
            }
        }

        Ok(())
    }

    fn swap_nodes_leave_children(
        &mut self,
        first_id: &NodeId,
        second_id: &NodeId,
    ) -> Result<(), NodeIdError> {
        //take care of these nodes' children's parent values
        self.set_parent_of_children(first_id, Some(second_id.clone()));
        self.set_parent_of_children(second_id, Some(first_id.clone()));

        //swap children of these nodes
        let first_children = self.core_tree.get_unsafe(first_id).children().clone();
        let second_children = self.core_tree.get_unsafe(second_id).children().clone();
        self.core_tree.get_mut_unsafe(first_id).set_children(
            second_children,
        );
        self.core_tree.get_mut_unsafe(second_id).set_children(
            first_children,
        );

        let first_parent = self.core_tree.get_unsafe(first_id).parent().cloned();
        let second_parent = self.core_tree.get_unsafe(second_id).parent().cloned();

        //todo: some of this could probably be abstracted out into a method or two
        match (first_parent, second_parent) {
            (Some(ref first_parent_id), Some(ref second_parent_id)) => {
                let first_index = self.core_tree
                    .get_unsafe(first_parent_id)
                    .children()
                    .iter()
                    .position(|id| id == first_id)
                    .unwrap();
                let second_index = self.core_tree
                    .get_unsafe(second_parent_id)
                    .children()
                    .iter()
                    .position(|id| id == second_id)
                    .unwrap();

                unsafe {
                    let temp = self.core_tree
                        .get_mut_unsafe(first_parent_id)
                        .children_mut()
                        .get_unchecked_mut(first_index);
                    *temp = second_id.clone();
                }
                unsafe {
                    let temp = self.core_tree
                        .get_mut_unsafe(second_parent_id)
                        .children_mut()
                        .get_unchecked_mut(second_index);
                    *temp = first_id.clone();
                }

                self.core_tree.get_mut_unsafe(first_id).set_parent(Some(
                    second_parent_id
                        .clone(),
                ));
                self.core_tree.get_mut_unsafe(second_id).set_parent(Some(
                    first_parent_id
                        .clone(),
                ));
            }
            (Some(ref first_parent_id), None) => {
                let first_index = self.core_tree
                    .get_unsafe(first_parent_id)
                    .children()
                    .iter()
                    .position(|id| id == first_id)
                    .unwrap();

                unsafe {
                    let temp = self.core_tree
                        .get_mut_unsafe(first_parent_id)
                        .children_mut()
                        .get_unchecked_mut(first_index);
                    *temp = second_id.clone();
                }

                self.core_tree.get_mut_unsafe(first_id).set_parent(None);
                self.core_tree.get_mut_unsafe(second_id).set_parent(Some(
                    first_parent_id
                        .clone(),
                ));

                if let Some(root_id) = self.root_node_id().cloned() {
                    if root_id == second_id.clone() {
                        self.core_tree.root = Some(first_id.clone());
                    }
                }
            }
            (None, Some(ref second_parent_id)) => {
                let second_index = self.core_tree
                    .get_unsafe(second_parent_id)
                    .children()
                    .iter()
                    .position(|id| id == second_id)
                    .unwrap();

                unsafe {
                    let temp = self.core_tree
                        .get_mut_unsafe(second_parent_id)
                        .children_mut()
                        .get_unchecked_mut(second_index);
                    *temp = first_id.clone();
                }

                self.core_tree.get_mut_unsafe(first_id).set_parent(Some(
                    second_parent_id
                        .clone(),
                ));
                self.core_tree.get_mut_unsafe(second_id).set_parent(None);

                if let Some(root_id) = self.root_node_id().cloned() {
                    if root_id == first_id.clone() {
                        self.core_tree.root = Some(second_id.clone());
                    }
                }
            }
            (None, None) => {
                if let Some(root_id) = self.root_node_id().cloned() {

                    if root_id == first_id.clone() {
                        self.core_tree.root = Some(second_id.clone());
                    } else if root_id == second_id.clone() {
                        self.core_tree.root = Some(first_id.clone());
                    }
                }
            }
        }

        Ok(())
    }

    fn swap_nodes_children_only(
        &mut self,
        first_id: &NodeId,
        second_id: &NodeId,
    ) -> Result<(), NodeIdError> {
        let lower_upper_test = self.find_subtree_root_between_ids(first_id, second_id)
            .map(|_| (first_id, second_id))
            .or_else(|| {
                self.find_subtree_root_between_ids(second_id, first_id)
                    .map(|_| (second_id, first_id))
            });

        // todo: lots of repetition in here

        let first_children = self.core_tree.get_unsafe(first_id).children().clone();
        let second_children = self.core_tree.get_unsafe(second_id).children().clone();

        if let Some((lower_id, upper_id)) = lower_upper_test {

            let lower_parent = self.core_tree
                .get_unsafe(lower_id)
                .parent()
                .cloned()
                .unwrap();

            let (mut upper_children, lower_children) = if upper_id == first_id {
                (first_children, second_children)
            } else {
                (second_children, first_children)
            };

            for child in &upper_children {
                self.core_tree.get_mut_unsafe(child).set_parent(
                    Some(lower_id.clone()),
                );
            }
            for child in &lower_children {
                self.core_tree.get_mut_unsafe(child).set_parent(
                    Some(upper_id.clone()),
                );
            }

            if upper_id == &lower_parent {
                // direct child
                upper_children.retain(|id| id != lower_id);
            }

            //swap children of these nodes
            self.core_tree.get_mut_unsafe(upper_id).set_children(
                lower_children,
            );
            self.core_tree.get_mut_unsafe(lower_id).set_children(
                upper_children,
            );

            //add lower to upper
            self.set_as_parent_and_child(upper_id, lower_id);

        } else {
            //just across

            //take care of these nodes' children's parent values
            for child in &first_children {
                self.core_tree.get_mut_unsafe(child).set_parent(
                    Some(second_id.clone()),
                );
            }
            for child in &second_children {
                self.core_tree.get_mut_unsafe(child).set_parent(
                    Some(first_id.clone()),
                );
            }

            //swap children of these nodes
            self.core_tree.get_mut_unsafe(first_id).set_children(
                second_children,
            );
            self.core_tree.get_mut_unsafe(second_id).set_children(
                first_children,
            );
        }

        Ok(())
    }

    fn find_subtree_root_between_ids<'f>(
        &'f self,
        lower_id: &'f NodeId,
        upper_id: &'f NodeId,
    ) -> Option<&'f NodeId> {
        if let Some(lower_parent) = self.core_tree.get_unsafe(lower_id).parent() {
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
        self.core_tree.get_mut_unsafe(parent_id).add_child(
            child_id.clone(),
        );

        self.core_tree.get_mut_unsafe(child_id).set_parent(Some(
            parent_id.clone(),
        ));
    }

    fn detach_from_parent(&mut self, parent_id: &NodeId, node_id: &NodeId) {
        self.core_tree
            .get_mut_unsafe(parent_id)
            .children_mut()
            .retain(|child_id| child_id != node_id);
    }

    fn remove_node_internal(&mut self, node_id: NodeId) -> VecNode<T> {

        let mut node = self.core_tree.remove(node_id.clone());

        // The only thing we care about here is dealing with "this" Node's parent's children
        // This Node's children's parent will be handled in different ways depending upon how this
        // method is called.
        if let Some(parent_id) = node.parent() {
            self.core_tree
                .get_mut_unsafe(parent_id)
                .children_mut()
                .retain(|child_id| child_id != &node_id);
        }

        // avoid providing the caller with extra copies of NodeIds
        node.children_mut().clear();
        node.set_parent(None);

        node
    }

    fn clear_parent(&mut self, node_id: &NodeId) {
        self.set_parent(node_id, None);
    }

    fn set_parent(&mut self, node_id: &NodeId, new_parent: Option<NodeId>) {
        self.core_tree.get_mut_unsafe(node_id).set_parent(
            new_parent,
        );
    }

    fn clear_parent_of_children(&mut self, node_id: &NodeId) {
        self.set_parent_of_children(node_id, None);
    }

    fn set_parent_of_children(&mut self, node_id: &NodeId, new_parent: Option<NodeId>) {
        for child_id in self.core_tree.get_unsafe(node_id).children().clone() {
            self.set_parent(&child_id, new_parent.clone());
        }
    }

    pub(crate) fn core_tree(&self) -> &CoreTree<VecNode<T>, T> {
        &self.core_tree
    }

    pub(crate) fn core_tree_mut(&mut self) -> &mut CoreTree<VecNode<T>, T> {
        &mut self.core_tree
    }
}

#[cfg(test)]
mod tree_builder_tests {
    use ::*;

    #[test]
    fn test_new() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new();
        assert!(tb.root.is_none());
        assert_eq!(tb.node_capacity, 0);
        assert_eq!(tb.swap_capacity, 0);
    }

    #[test]
    fn test_with_root() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new().with_root(Node::new(5));

        assert_eq!(tb.root.unwrap().data(), &5);
        assert_eq!(tb.node_capacity, 0);
        assert_eq!(tb.swap_capacity, 0);
    }

    #[test]
    fn test_with_node_capacity() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new().with_node_capacity(10);

        assert!(tb.root.is_none());
        assert_eq!(tb.node_capacity, 10);
        assert_eq!(tb.swap_capacity, 0);
    }

    #[test]
    fn test_with_swap_capacity() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new().with_swap_capacity(10);

        assert!(tb.root.is_none());
        assert_eq!(tb.node_capacity, 0);
        assert_eq!(tb.swap_capacity, 10);
    }

    #[test]
    fn test_with_all_settings() {
        let tb: VecTreeBuilder<i32> = VecTreeBuilder::new()
            .with_root(Node::new(5))
            .with_node_capacity(10)
            .with_swap_capacity(3);

        assert_eq!(tb.root.unwrap().data(), &5);
        assert_eq!(tb.node_capacity, 10);
        assert_eq!(tb.swap_capacity, 3);
    }

    #[test]
    fn test_build() {
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

#[cfg(test)]
mod tree_tests {
    use ::*;

    #[test]
    fn test_new() {
        let tree: VecTree<i32> = VecTree::new();

        assert_eq!(tree.core_tree.root, None);
        assert_eq!(tree.core_tree.nodes.len(), 0);
        assert_eq!(tree.core_tree.free_ids.len(), 0);
    }

    #[test]
    fn test_get() {
        let tree = VecTreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.core_tree.root.clone().unwrap();
        let root = tree.get(&root_id).unwrap();

        assert_eq!(root.data(), &5);
    }

    #[test]
    fn test_get_mut() {
        let mut tree = VecTreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.core_tree.root.clone().unwrap();

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
        use InsertBehavior::*;

        let a = 5;
        let b = 6;
        let node_a = Node::new(a);
        let node_b = Node::new(b);

        let mut tree = VecTreeBuilder::new().build();

        let node_a_id = tree.insert(node_a, AsRoot).unwrap();
        let root_id = tree.core_tree.root.clone().unwrap();
        assert_eq!(node_a_id, root_id);

        {
            let node_a_ref = tree.get(&node_a_id).unwrap();
            let root_ref = tree.get(&root_id).unwrap();
            assert_eq!(node_a_ref.data(), &a);
            assert_eq!(root_ref.data(), &a);
        }

        let node_b_id = tree.insert(node_b, AsRoot).unwrap();
        let root_id = tree.core_tree.root.clone().unwrap();
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
        let tree = VecTreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.core_tree.root.clone().unwrap();
        let root_node_id = tree.root_node_id().unwrap();

        assert_eq!(&root_id, root_node_id);
    }

    #[test]
    fn test_insert_under_node() {
        use InsertBehavior::*;

        let a = 1;
        let b = 2;
        let r = 5;

        let mut tree = VecTreeBuilder::new().with_root(Node::new(r)).build();

        let node_a = Node::new(a);
        let node_b = Node::new(b);

        let root_id = tree.core_tree.root.clone().unwrap();
        let node_a_id = tree.insert(node_a, UnderNode(&root_id)).unwrap();
        let node_b_id = tree.insert(node_b, UnderNode(&root_id)).unwrap();

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
    fn test_remove_lift_children() {
        use InsertBehavior::*;
        use RemoveBehavior::*;

        let mut tree = VecTreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.core_tree.root.clone().unwrap();

        let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2_id = tree.insert(Node::new(2), UnderNode(&node_1_id)).unwrap();
        let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();

        let node_1 = tree.remove(node_1_id.clone(), LiftChildren).unwrap();

        assert_eq!(Some(&root_id), tree.root_node_id());

        assert_eq!(node_1.data(), &1);
        assert_eq!(node_1.children().len(), 0);
        assert!(node_1.parent().is_none());
        assert!(tree.get(&node_1_id).is_err());

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
    fn test_remove_orphan_children() {
        use InsertBehavior::*;
        use RemoveBehavior::*;

        let mut tree = VecTreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.core_tree.root.clone().unwrap();

        let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2_id = tree.insert(Node::new(2), UnderNode(&node_1_id)).unwrap();
        let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();

        let node_1 = tree.remove(node_1_id.clone(), OrphanChildren).unwrap();

        assert_eq!(Some(&root_id), tree.root_node_id());

        assert_eq!(node_1.data(), &1);
        assert_eq!(node_1.children().len(), 0);
        assert!(node_1.parent().is_none());
        assert!(tree.get(&node_1_id).is_err());

        let node_2_ref = tree.get(&node_2_id).unwrap();
        let node_3_ref = tree.get(&node_3_id).unwrap();

        assert_eq!(node_2_ref.data(), &2);
        assert_eq!(node_3_ref.data(), &3);

        assert!(node_2_ref.parent().is_none());
        assert!(node_3_ref.parent().is_none());
    }

    #[test]
    fn test_remove_root() {
        use RemoveBehavior::*;

        let mut tree = VecTreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.core_tree.root.clone().unwrap();
        tree.remove(root_id.clone(), OrphanChildren).unwrap();
        assert_eq!(None, tree.root_node_id());

        let mut tree = VecTreeBuilder::new().with_root(Node::new(5)).build();

        let root_id = tree.core_tree.root.clone().unwrap();
        tree.remove(root_id.clone(), LiftChildren).unwrap();
        assert_eq!(None, tree.root_node_id());
    }

    #[test]
    fn test_move_node_to_parent() {
        use InsertBehavior::*;
        use MoveBehavior::*;

        let mut tree = VecTree::new();

        let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
        let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();

        // move 3 "across" the tree
        tree.move_node(&node_3_id, ToParent(&node_2_id)).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&node_2_id).unwrap().children().contains(
            &node_3_id,
        ));

        // move 3 "up" the tree
        tree.move_node(&node_3_id, ToParent(&root_id)).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_3_id));

        // move 3 "down" (really this is across though) the tree
        tree.move_node(&node_3_id, ToParent(&node_1_id)).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&node_1_id).unwrap().children().contains(
            &node_3_id,
        ));

        // move 1 "down" the tree
        tree.move_node(&node_1_id, ToParent(&node_3_id)).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_3_id));
        assert!(tree.get(&node_3_id).unwrap().children().contains(
            &node_1_id,
        ));

        // note: node_1 is at the lowest point in the tree before these insertions.
        let node_4_id = tree.insert(Node::new(4), UnderNode(&node_1_id)).unwrap();
        let node_5_id = tree.insert(Node::new(5), UnderNode(&node_4_id)).unwrap();

        // move 3 "down" the tree
        tree.move_node(&node_3_id, ToParent(&node_5_id)).unwrap();
        assert!(tree.get(&root_id).unwrap().children().contains(&node_2_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&node_1_id).unwrap().children().contains(
            &node_4_id,
        ));
        assert!(tree.get(&node_4_id).unwrap().children().contains(
            &node_5_id,
        ));
        assert!(tree.get(&node_5_id).unwrap().children().contains(
            &node_3_id,
        ));

        // move root "down" the tree
        tree.move_node(&root_id, ToParent(&node_2_id)).unwrap();
        assert!(tree.get(&node_2_id).unwrap().children().contains(&root_id));
        assert!(tree.get(&root_id).unwrap().children().contains(&node_1_id));
        assert!(tree.get(&node_1_id).unwrap().children().contains(
            &node_4_id,
        ));
        assert!(tree.get(&node_4_id).unwrap().children().contains(
            &node_5_id,
        ));
        assert!(tree.get(&node_5_id).unwrap().children().contains(
            &node_3_id,
        ));
        assert_eq!(tree.root_node_id(), Some(&node_2_id));
    }

    #[test]
    fn test_move_node_to_root() {
        use InsertBehavior::*;

        // test move with existing root
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&node_1_id)).unwrap();

            tree.move_node_to_root(&node_2_id).unwrap();

            assert_eq!(tree.root_node_id(), Some(&node_2_id));
            assert!(tree.get(&node_2_id).unwrap().children().contains(&root_id));
            assert!(!tree.get(&node_1_id).unwrap().children().contains(
                &node_2_id,
            ));
        }

        // test move with existing root and with orphan
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&node_1_id)).unwrap();

            tree.remove_node_orphan_children(node_1_id).unwrap();
            tree.move_node_to_root(&node_2_id).unwrap();

            assert_eq!(tree.root_node_id(), Some(&node_2_id));
            assert!(tree.get(&node_2_id).unwrap().children().contains(&root_id));
            assert_eq!(tree.get(&root_id).unwrap().children().len(), 0);
        }

        // test move without root and with orphan
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&node_1_id)).unwrap();

            tree.remove_node_orphan_children(root_id).unwrap();
            tree.move_node_to_root(&node_1_id).unwrap();

            assert_eq!(tree.root_node_id(), Some(&node_1_id));
            assert!(tree.get(&node_1_id).unwrap().children().contains(
                &node_2_id,
            ));
            assert_eq!(tree.get(&node_1_id).unwrap().children().len(), 1);
        }
    }

    #[test]
    fn test_find_subtree_root_below_upper_id() {
        use InsertBehavior::*;

        let mut tree = VecTree::new();

        let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2_id = tree.insert(Node::new(2), UnderNode(&node_1_id)).unwrap();
        let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
        let node_4_id = tree.insert(Node::new(4), UnderNode(&node_2_id)).unwrap();

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
    fn test_swap_nodes_take_children() {
        use InsertBehavior::*;
        use SwapBehavior::*;

        // test across swap
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            let node_4_id = tree.insert(Node::new(4), UnderNode(&node_2_id)).unwrap();

            tree.swap_nodes(&node_3_id, &node_4_id, TakeChildren)
                .unwrap();

            assert!(tree.get(&node_1_id).unwrap().children().contains(
                &node_4_id,
            ));
            assert!(tree.get(&node_2_id).unwrap().children().contains(
                &node_3_id,
            ));
        }

        // test ordering via swap
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();

            tree.swap_nodes(&node_1_id, &node_2_id, TakeChildren)
                .unwrap();

            let children = tree.get(&root_id).unwrap().children();
            assert!(children[0] == node_2_id);
            assert!(children[1] == node_1_id);
        }

        // test swap down
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();

            tree.swap_nodes(&root_id, &node_3_id, TakeChildren).unwrap();

            assert_eq!(tree.root_node_id(), Some(&node_3_id));

            assert!(tree.get(&node_3_id).unwrap().children().contains(&root_id));

            let children = tree.get(&root_id).unwrap().children();
            assert!(children[0] == node_1_id);
            assert!(children[1] == node_2_id);
        }

        // test swap down without root
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();

            tree.swap_nodes(&node_1_id, &node_3_id, TakeChildren)
                .unwrap();

            assert!(tree.get(&node_3_id).unwrap().children().contains(
                &node_1_id,
            ));

            let children = tree.get(&root_id).unwrap().children();
            assert!(children[0] == node_3_id);
            assert!(children[1] == node_2_id);
        }
    }

    #[test]
    fn test_swap_nodes_leave_children() {
        use InsertBehavior::*;
        use SwapBehavior::*;
        use MoveBehavior::*;
        use RemoveBehavior::*;

        // test across swap
        // from:
        //        0
        //       / \
        //      1   2
        //      |   |
        //      3   4
        // to:
        //        0
        //       / \
        //      2   1
        //      |   |
        //      3   4
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            let node_4_id = tree.insert(Node::new(4), UnderNode(&node_2_id)).unwrap();

            tree.swap_nodes(&node_1_id, &node_2_id, LeaveChildren)
                .unwrap();

            let root_children = tree.get(&root_id).unwrap().children();
            assert_eq!(root_children[0], node_2_id);
            assert_eq!(root_children[1], node_1_id);

            assert_eq!(tree.get(&node_3_id).unwrap().parent(), Some(&node_2_id));
            assert_eq!(tree.get(&node_4_id).unwrap().parent(), Some(&node_1_id));

            assert!(tree.get(&node_1_id).unwrap().children().contains(
                &node_4_id,
            ));
            assert!(tree.get(&node_2_id).unwrap().children().contains(
                &node_3_id,
            ));
        }

        // test down swap (with no space between nodes)
        // from:
        //        0
        //       / \
        //      1   2
        //      |   |
        //      3   4
        // to:
        //        0
        //       / \
        //      3   2
        //      |   |
        //      1   4
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            tree.insert(Node::new(4), UnderNode(&node_2_id)).unwrap();

            tree.swap_nodes(&node_1_id, &node_3_id, LeaveChildren)
                .unwrap();

            let root_children = tree.get(&root_id).unwrap().children();
            assert_eq!(root_children[0], node_3_id);
            assert_eq!(root_children[1], node_2_id);

            assert_eq!(tree.get(&node_3_id).unwrap().parent(), Some(&root_id));
            assert_eq!(tree.get(&node_1_id).unwrap().parent(), Some(&node_3_id));

            assert!(tree.get(&node_3_id).unwrap().children().contains(
                &node_1_id,
            ));
            assert_eq!(tree.get(&node_1_id).unwrap().children().len(), 0);
        }

        // test down swap (with space between nodes)
        // from:
        //        0
        //       / \
        //      1   2
        //      |   |
        //      3   4
        //      |
        //      5
        // to:
        //        0
        //       / \
        //      5   2
        //      |   |
        //      3   4
        //      |
        //      1
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            tree.insert(Node::new(4), UnderNode(&node_2_id)).unwrap();
            let node_5_id = tree.insert(Node::new(5), UnderNode(&node_3_id)).unwrap();

            tree.swap_nodes(&node_1_id, &node_5_id, LeaveChildren)
                .unwrap();

            let root_children = tree.get(&root_id).unwrap().children();
            assert_eq!(root_children[0], node_5_id);
            assert_eq!(root_children[1], node_2_id);

            assert_eq!(tree.get(&node_3_id).unwrap().parent(), Some(&node_5_id));
            assert_eq!(tree.get(&node_1_id).unwrap().parent(), Some(&node_3_id));
            assert_eq!(tree.get(&node_5_id).unwrap().parent(), Some(&root_id));

            assert!(tree.get(&node_3_id).unwrap().children().contains(
                &node_1_id,
            ));
            assert!(tree.get(&node_5_id).unwrap().children().contains(
                &node_3_id,
            ));
            assert_eq!(tree.get(&node_1_id).unwrap().children().len(), 0);
        }

        // test down swap (with root)
        // from:
        //        0
        //       / \
        //      1   2
        //      |   |
        //      3   4
        // to:
        //        4
        //       / \
        //      1   2
        //      |   |
        //      3   0
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            let node_4_id = tree.insert(Node::new(4), UnderNode(&node_2_id)).unwrap();

            tree.swap_nodes(&root_id, &node_4_id, LeaveChildren)
                .unwrap();

            assert_eq!(tree.root_node_id(), Some(&node_4_id));

            let node_4_children = tree.get(&node_4_id).unwrap().children();
            assert_eq!(node_4_children[0], node_1_id);
            assert_eq!(node_4_children[1], node_2_id);

            assert_eq!(tree.get(&node_1_id).unwrap().parent(), Some(&node_4_id));
            assert_eq!(tree.get(&node_2_id).unwrap().parent(), Some(&node_4_id));
            assert_eq!(tree.get(&root_id).unwrap().parent(), Some(&node_2_id));

            assert!(tree.get(&node_2_id).unwrap().children().contains(&root_id));
            assert_eq!(tree.get(&root_id).unwrap().children().len(), 0);
        }

        // test orphaned swap (no root)
        // from:
        //      1   2
        //      |   |
        //      3   4
        // to:
        //      2   1
        //      |   |
        //      3   4
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            let node_4_id = tree.insert(Node::new(4), UnderNode(&node_2_id)).unwrap();
            tree.remove(root_id, OrphanChildren).unwrap();

            tree.swap_nodes(&node_1_id, &node_2_id, LeaveChildren)
                .unwrap();

            assert_eq!(tree.root_node_id(), None);

            assert_eq!(tree.get(&node_3_id).unwrap().parent(), Some(&node_2_id));
            assert_eq!(tree.get(&node_4_id).unwrap().parent(), Some(&node_1_id));

            assert!(tree.get(&node_2_id).unwrap().children().contains(
                &node_3_id,
            ));
            assert!(tree.get(&node_1_id).unwrap().children().contains(
                &node_4_id,
            ));
        }

        // test orphaned swap (1 is root)
        // from:
        //      1   2
        //      |   |
        //      3   4
        // to:
        //      2   1
        //      |   |
        //      3   4
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            let node_4_id = tree.insert(Node::new(4), UnderNode(&node_2_id)).unwrap();
            tree.remove(root_id, OrphanChildren).unwrap();
            tree.move_node(&node_1_id, ToRoot).unwrap();

            tree.swap_nodes(&node_1_id, &node_2_id, LeaveChildren)
                .unwrap();

            assert_eq!(tree.root_node_id(), Some(&node_2_id));

            assert_eq!(tree.get(&node_3_id).unwrap().parent(), Some(&node_2_id));
            assert_eq!(tree.get(&node_4_id).unwrap().parent(), Some(&node_1_id));

            assert!(tree.get(&node_2_id).unwrap().children().contains(
                &node_3_id,
            ));
            assert!(tree.get(&node_1_id).unwrap().children().contains(
                &node_4_id,
            ));
        }
    }

    #[test]
    fn test_swap_nodes_children_only() {
        use InsertBehavior::*;
        use SwapBehavior::*;

        // test across swap
        // swap(1,2)
        // from:
        //        0
        //       / \
        //      1   2
        //     / \   \
        //    3   4   5
        // to:
        //        0
        //       / \
        //      1   2
        //     /   / \
        //    5   3   4
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            let node_4_id = tree.insert(Node::new(4), UnderNode(&node_1_id)).unwrap();
            let node_5_id = tree.insert(Node::new(5), UnderNode(&node_2_id)).unwrap();

            tree.swap_nodes(&node_1_id, &node_2_id, ChildrenOnly)
                .unwrap();

            let root_children = tree.get(&root_id).unwrap().children();
            assert_eq!(root_children[0], node_1_id);
            assert_eq!(root_children[1], node_2_id);

            assert_eq!(tree.get(&node_3_id).unwrap().parent(), Some(&node_2_id));
            assert_eq!(tree.get(&node_4_id).unwrap().parent(), Some(&node_2_id));
            assert_eq!(tree.get(&node_5_id).unwrap().parent(), Some(&node_1_id));

            assert!(tree.get(&node_1_id).unwrap().children().contains(
                &node_5_id,
            ));
            assert!(tree.get(&node_2_id).unwrap().children().contains(
                &node_3_id,
            ));
            assert!(tree.get(&node_2_id).unwrap().children().contains(
                &node_4_id,
            ));
        }

        // test down swap (with no space between nodes)
        // swap(1,3)
        // from:
        //        0
        //       / \
        //      1   2
        //     / \   \
        //    3   4   5
        //    |   |
        //    6   7
        // to:
        //        0
        //       / \
        //      1   2
        //     / \   \
        //    6   3   5
        //        |
        //        4
        //        |
        //        7
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            let node_4_id = tree.insert(Node::new(4), UnderNode(&node_1_id)).unwrap();
            tree.insert(Node::new(5), UnderNode(&node_2_id)).unwrap();
            let node_6_id = tree.insert(Node::new(6), UnderNode(&node_3_id)).unwrap();
            tree.insert(Node::new(7), UnderNode(&node_4_id)).unwrap();

            tree.swap_nodes(&node_1_id, &node_3_id, ChildrenOnly)
                .unwrap();

            let root_children = tree.get(&root_id).unwrap().children();
            assert_eq!(root_children[0], node_1_id);
            assert_eq!(root_children[1], node_2_id);

            assert_eq!(tree.get(&node_3_id).unwrap().parent(), Some(&node_1_id));
            assert_eq!(tree.get(&node_1_id).unwrap().parent(), Some(&root_id));
            assert_eq!(tree.get(&node_4_id).unwrap().parent(), Some(&node_3_id));
            assert_eq!(tree.get(&node_6_id).unwrap().parent(), Some(&node_1_id));

            let node_1_children = tree.get(&node_1_id).unwrap().children();
            assert_eq!(node_1_children[0], node_6_id);
            assert_eq!(node_1_children[1], node_3_id);
            assert!(tree.get(&node_3_id).unwrap().children().contains(
                &node_4_id,
            ));
        }

        // test down swap (with space between nodes)
        // swap(1, 6)
        // from:
        //        0
        //       / \
        //      1   2
        //     / \   \
        //    3   4   5
        //    |   |
        //    6   7
        // to:
        //        0
        //       / \
        //      1   2
        //     /     \
        //    6       5
        //   / \
        //  3   4
        //      |
        //      7
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            let node_4_id = tree.insert(Node::new(4), UnderNode(&node_1_id)).unwrap();
            tree.insert(Node::new(5), UnderNode(&node_2_id)).unwrap();
            let node_6_id = tree.insert(Node::new(6), UnderNode(&node_3_id)).unwrap();
            tree.insert(Node::new(7), UnderNode(&node_4_id)).unwrap();

            tree.swap_nodes(&node_1_id, &node_6_id, ChildrenOnly)
                .unwrap();

            let root_children = tree.get(&root_id).unwrap().children();
            assert_eq!(root_children[0], node_1_id);
            assert_eq!(root_children[1], node_2_id);

            assert_eq!(tree.get(&node_3_id).unwrap().parent(), Some(&node_6_id));
            assert_eq!(tree.get(&node_4_id).unwrap().parent(), Some(&node_6_id));
            assert_eq!(tree.get(&node_6_id).unwrap().parent(), Some(&node_1_id));

            assert!(tree.get(&node_1_id).unwrap().children().contains(
                &node_6_id,
            ));
            assert!(!tree.get(&node_1_id).unwrap().children().contains(
                &node_3_id,
            ));
            assert!(!tree.get(&node_1_id).unwrap().children().contains(
                &node_4_id,
            ));
            assert!(tree.get(&node_6_id).unwrap().children().contains(
                &node_3_id,
            ));
            assert!(tree.get(&node_6_id).unwrap().children().contains(
                &node_4_id,
            ));
        }

        // test down swap (with root)
        // swap(0,1)
        // from:
        //        0
        //       / \
        //      1   2
        //     / \   \
        //    3   4   5
        //    |   |
        //    6   7
        // to:
        //        0
        //       /|\
        //      3 4 1
        //      | | |
        //      6 7 2
        //          |
        //          5
        {
            let mut tree = VecTree::new();
            let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
            let node_1_id = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
            let node_2_id = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
            let node_3_id = tree.insert(Node::new(3), UnderNode(&node_1_id)).unwrap();
            let node_4_id = tree.insert(Node::new(4), UnderNode(&node_1_id)).unwrap();
            tree.insert(Node::new(5), UnderNode(&node_2_id)).unwrap();
            tree.insert(Node::new(6), UnderNode(&node_3_id)).unwrap();
            tree.insert(Node::new(7), UnderNode(&node_4_id)).unwrap();

            tree.swap_nodes(&root_id, &node_1_id, ChildrenOnly).unwrap();

            let root_children = tree.get(&root_id).unwrap().children();
            assert_eq!(root_children[0], node_3_id);
            assert_eq!(root_children[1], node_4_id);
            assert_eq!(root_children[2], node_1_id);

            assert_eq!(tree.get(&node_1_id).unwrap().parent(), Some(&root_id));
            assert_eq!(tree.get(&node_3_id).unwrap().parent(), Some(&root_id));
            assert_eq!(tree.get(&node_4_id).unwrap().parent(), Some(&root_id));
            assert_eq!(tree.get(&node_2_id).unwrap().parent(), Some(&node_1_id));

            let node_1_children = tree.get(&node_1_id).unwrap().children();
            assert_eq!(node_1_children[0], node_2_id);
        }
    }
}
