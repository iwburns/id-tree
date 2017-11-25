use std::cmp::Ordering;
use std::marker::PhantomData;
use ::*;

use super::core::CoreTree;

///
/// An `OptTree` builder that provides more control over how a `OptTree` is created.
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
    pub fn build<'a>(self) -> OptTree<'a, T> {
        OptTree {
            core_tree: CoreTree::new(self.root, self.node_capacity, self.swap_capacity),
            phantom: PhantomData,
        }
    }
}

pub struct OptTree<'a, T: 'a> {
    core_tree: CoreTree<OptNode<T>, T>,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> Tree<'a, T> for OptTree<'a, T> {
    type NodeType = OptNode<T>;
    type AncestorsIter = Ancestors<'a, OptTree<'a, T>, T>;
    type AncestorIdsIter = AncestorIds<'a, OptTree<'a, T>, T>;
    type ChildrenIter = OptChildren<'a, T>;
    type ChildrenIdsIter = OptChildrenIds<'a, T>;

    // todo: make real iterators for these.
    type PreOrderIter = Ancestors<'a, OptTree<'a, T>, T>;
    type PostOrderIter = Ancestors<'a, OptTree<'a, T>, T>;
    type LevelOrderIter = Ancestors<'a, OptTree<'a, T>, T>;

    fn new() -> Self {
        OptTreeBuilder::new().build()
    }

    fn height(&self) -> usize {
        unimplemented!()
    }

    fn insert(
        &mut self,
        node: OptNode<T>,
        behavior: InsertBehavior,
    ) -> Result<NodeId, NodeIdError> {
        match behavior {
            InsertBehavior::UnderNode(parent_id) => {
                self.core_tree.validate_node_id(parent_id)?;
                self.insert_under_node(node, parent_id)
            }
            InsertBehavior::AsRoot => Ok(self.set_root(node)),
        }
    }

    //todo: test
    fn get(&self, node_id: &NodeId) -> Result<&OptNode<T>, NodeIdError> {
        self.core_tree.get(node_id)
    }

    fn get_mut(&mut self, node_id: &NodeId) -> Result<&mut OptNode<T>, NodeIdError> {
        unimplemented!()
    }

    //todo: test
    unsafe fn get_unchecked(&self, node_id: &NodeId) -> &OptNode<T> {
        self.core_tree
            .nodes
            .get_unchecked(node_id.index)
            .as_ref()
            .expect("Called VecTree.get_unchecked() with an invalid NodeId.")
    }

    //todo: test
    unsafe fn get_unchecked_mut(&mut self, node_id: &NodeId) -> &mut OptNode<T> {
        self.core_tree
            .nodes
            .get_unchecked_mut(node_id.index)
            .as_mut()
            .expect("Called VecTree.get_unchecked_mut() with an invalid NodeId.")
    }

    fn remove(
        &mut self,
        node_id: NodeId,
        behavior: RemoveBehavior,
    ) -> Result<OptNode<T>, NodeIdError> {
        unimplemented!()
    }

    fn move_node(&mut self, node_id: &NodeId, behavior: MoveBehavior) -> Result<(), NodeIdError> {
        unimplemented!()
    }

    fn sort_children_by<F>(&mut self, node_id: &NodeId, compare: F) -> Result<(), NodeIdError>
    where
        F: FnMut(&OptNode<T>, &OptNode<T>) -> Ordering,
    {
        unimplemented!()
    }

    fn sort_children_by_data(&mut self, node_id: &NodeId) -> Result<(), NodeIdError>
    where
        T: Ord,
    {
        unimplemented!()
    }

    fn sort_children_by_key<K, F>(&mut self, node_id: &NodeId, f: F) -> Result<(), NodeIdError>
    where
        K: Ord,
        F: FnMut(&OptNode<T>) -> K,
    {
        unimplemented!()
    }

    fn swap_nodes(
        &mut self,
        first_id: &NodeId,
        second_id: &NodeId,
        behavior: SwapBehavior,
    ) -> Result<(), NodeIdError> {
        unimplemented!()
    }

    fn root_node_id(&self) -> Option<&NodeId> {
        unimplemented!()
    }

    fn ancestors(&'a self, node_id: &NodeId) -> Result<Self::AncestorsIter, NodeIdError> {
        self.core_tree.validate_node_id(node_id)?;
        Ok(Ancestors::new(self, node_id.clone()))
    }

    fn ancestor_ids(&'a self, node_id: &NodeId) -> Result<Self::AncestorIdsIter, NodeIdError> {
        unimplemented!()
    }

    fn children(&'a self, node_id: &NodeId) -> Result<Self::ChildrenIter, NodeIdError> {
        unimplemented!()
    }

    fn children_ids(&'a self, node_id: &NodeId) -> Result<Self::ChildrenIdsIter, NodeIdError> {
        unimplemented!()
    }

    fn traverse_pre_order(&'a self, node_id: &NodeId) -> Result<Self::PreOrderIter, NodeIdError> {
        unimplemented!()
    }

    fn traverse_post_order(&'a self, node_id: &NodeId) -> Result<Self::PostOrderIter, NodeIdError> {
        unimplemented!()
    }

    fn traverse_level_order(
        &'a self,
        node_id: &NodeId,
    ) -> Result<Self::LevelOrderIter, NodeIdError> {
        unimplemented!()
    }
}

impl<'a, T> OptTree<'a, T> {
    ///
    ///
    ///
    fn insert_under_node(
        &mut self,
        mut node: OptNode<T>,
        parent_id: &NodeId,
    ) -> Result<NodeId, NodeIdError> {

        node.set_parent(Some(parent_id.clone()));

        let new_id = self.core_tree.insert(node);

        let children = {
            let parent = unsafe { self.get_unchecked(parent_id) };
            (parent.first_child().cloned(), parent.last_child().cloned())
        };

        match children {
            (Some(_), Some(last_id)) => {
                {
                    let parent = unsafe { self.get_unchecked_mut(parent_id) };
                    parent.set_last_child(Some(new_id.clone()));
                }

                {
                    let new_node = unsafe { self.get_unchecked_mut(&new_id) };
                    new_node.set_prev_sibling(Some(last_id.clone()));
                }

                let last_child = unsafe { self.get_unchecked_mut(&last_id) };
                last_child.set_next_sibling(Some(new_id.clone()));
            }
            //todo: find a better error message for these.
            (Some(_), None) => panic!("Found an OptNode in an invalid state."),
            (None, Some(_)) => panic!("Found an OptNode in an invalid state."),
            (None, None) => {
                let parent = unsafe { self.get_unchecked_mut(parent_id) };
                parent.set_first_child(Some(new_id.clone()));
                parent.set_last_child(Some(new_id.clone()));
            }
        }

        Ok(new_id)
    }

    ///
    /// Sets the root of the `Tree`.
    ///
    fn set_root(&mut self, new_root: OptNode<T>) -> NodeId {

        let current_root = self.core_tree.root.clone();
        let new_root_id = self.core_tree.set_root(new_root);

        if let Some(current_root_id) = current_root {
            {
                let current_root = unsafe { self.get_unchecked_mut(&current_root_id) };
                current_root.set_parent(Some(new_root_id.clone()));
            }

            let root = unsafe { self.get_unchecked_mut(&new_root_id) };
            root.set_first_child(Some(current_root_id.clone()));
            root.set_last_child(Some(current_root_id.clone()));
        }

        new_root_id
    }
}

#[cfg(test)]
mod opt_tree_tests {
    use ::*;
    use ::behaviors::InsertBehavior::*;

    fn new_tree<'a>() -> (NodeId, OptTree<'a, i32>) {
        let tree = OptTreeBuilder::new()
            .with_root(Node::new(1))
            .with_node_capacity(2usize)
            .with_swap_capacity(3usize)
            .build();

        (tree.core_tree.root.clone().unwrap(), tree)
    }

    #[test]
    fn new() {
        let tree: OptTree<i32> = OptTree::new();

        assert_eq!(tree.core_tree.root, None);
        assert_eq!(tree.core_tree.nodes.len(), 0);
        assert_eq!(tree.core_tree.free_ids.len(), 0);
    }

    #[test]
    fn insert() {
        let (root_id, mut tree) = new_tree();

        assert_eq!(tree.get(&root_id).unwrap().parent(), None);
        assert_eq!(tree.get(&root_id).unwrap().first_child(), None);
        assert_eq!(tree.get(&root_id).unwrap().last_child(), None);
        assert_eq!(tree.get(&root_id).unwrap().prev_sibling(), None);
        assert_eq!(tree.get(&root_id).unwrap().next_sibling(), None);

        let child_1 = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();

        assert_eq!(tree.get(&root_id).unwrap().parent(), None);
        assert_eq!(tree.get(&root_id).unwrap().first_child(), Some(&child_1));
        assert_eq!(tree.get(&root_id).unwrap().last_child(), Some(&child_1));
        assert_eq!(tree.get(&root_id).unwrap().prev_sibling(), None);
        assert_eq!(tree.get(&root_id).unwrap().next_sibling(), None);

        assert_eq!(tree.get(&child_1).unwrap().parent(), Some(&root_id));
        assert_eq!(tree.get(&child_1).unwrap().first_child(), None);
        assert_eq!(tree.get(&child_1).unwrap().last_child(), None);
        assert_eq!(tree.get(&child_1).unwrap().prev_sibling(), None);
        assert_eq!(tree.get(&child_1).unwrap().next_sibling(), None);

        let child_2 = tree.insert(Node::new(3), UnderNode(&root_id)).unwrap();

        assert_eq!(tree.get(&root_id).unwrap().parent(), None);
        assert_eq!(tree.get(&root_id).unwrap().first_child(), Some(&child_1));
        assert_eq!(tree.get(&root_id).unwrap().last_child(), Some(&child_2));
        assert_eq!(tree.get(&root_id).unwrap().prev_sibling(), None);
        assert_eq!(tree.get(&root_id).unwrap().next_sibling(), None);

        assert_eq!(tree.get(&child_1).unwrap().parent(), Some(&root_id));
        assert_eq!(tree.get(&child_1).unwrap().first_child(), None);
        assert_eq!(tree.get(&child_1).unwrap().last_child(), None);
        assert_eq!(tree.get(&child_1).unwrap().prev_sibling(), None);
        assert_eq!(tree.get(&child_1).unwrap().next_sibling(), Some(&child_2));

        assert_eq!(tree.get(&child_2).unwrap().parent(), Some(&root_id));
        assert_eq!(tree.get(&child_2).unwrap().first_child(), None);
        assert_eq!(tree.get(&child_2).unwrap().last_child(), None);
        assert_eq!(tree.get(&child_2).unwrap().prev_sibling(), Some(&child_1));
        assert_eq!(tree.get(&child_2).unwrap().next_sibling(), None);

        let new_root = tree.insert(Node::new(0), AsRoot).unwrap();
        let old_root = root_id;

        assert_eq!(tree.get(&old_root).unwrap().parent(), Some(&new_root));
        assert_eq!(tree.get(&old_root).unwrap().first_child(), Some(&child_1));
        assert_eq!(tree.get(&old_root).unwrap().last_child(), Some(&child_2));
        assert_eq!(tree.get(&old_root).unwrap().prev_sibling(), None);
        assert_eq!(tree.get(&old_root).unwrap().next_sibling(), None);

        assert_eq!(tree.get(&new_root).unwrap().parent(), None);
        assert_eq!(tree.get(&new_root).unwrap().first_child(), Some(&old_root));
        assert_eq!(tree.get(&new_root).unwrap().last_child(), Some(&old_root));
        assert_eq!(tree.get(&new_root).unwrap().prev_sibling(), None);
        assert_eq!(tree.get(&new_root).unwrap().next_sibling(), None);

        assert_eq!(tree.get(&child_1).unwrap().parent(), Some(&old_root));
        assert_eq!(tree.get(&child_1).unwrap().first_child(), None);
        assert_eq!(tree.get(&child_1).unwrap().last_child(), None);
        assert_eq!(tree.get(&child_1).unwrap().prev_sibling(), None);
        assert_eq!(tree.get(&child_1).unwrap().next_sibling(), Some(&child_2));

        assert_eq!(tree.get(&child_2).unwrap().parent(), Some(&old_root));
        assert_eq!(tree.get(&child_2).unwrap().first_child(), None);
        assert_eq!(tree.get(&child_2).unwrap().last_child(), None);
        assert_eq!(tree.get(&child_2).unwrap().prev_sibling(), Some(&child_1));
        assert_eq!(tree.get(&child_2).unwrap().next_sibling(), None);
    }
}