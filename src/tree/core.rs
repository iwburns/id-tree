use std::marker::PhantomData;
use snowflake::ProcessUniqueId;

use ::*;

pub struct CoreTree<N, T>
where
    N: Node<T>,
{
    pub id: ProcessUniqueId,
    pub root: Option<NodeId>,
    pub nodes: Vec<Option<N>>,
    pub free_ids: Vec<NodeId>,
    phantom: PhantomData<T>,
}

impl<N, T> CoreTree<N, T>
where
    N: Node<T>,
{
    ///
    /// Creates a new CoreTree with the specified parameters and returns it.
    ///
    pub fn new(mut root: Option<N>, node_capacity: usize, swap_capacity: usize) -> CoreTree<N, T> {

        let tree_id = ProcessUniqueId::new();

        let mut tree = CoreTree {
            id: tree_id,
            root: None,
            nodes: Vec::with_capacity(node_capacity),
            free_ids: Vec::with_capacity(swap_capacity),
            phantom: PhantomData,
        };

        if root.is_some() {

            let node_id = NodeId { tree_id, index: 0 };

            tree.nodes.push(root.take());
            tree.root = Some(node_id);
        }

        tree
    }

    ///
    /// Sets the root of the `Tree`.
    ///
    pub fn set_root(&mut self, new_root: N) -> NodeId {
        let new_root_id = self.insert_new_node(new_root);
        self.root = Some(new_root_id.clone());
        new_root_id
    }

    ///
    /// Inserts a new node and returns a NodeId that points to it.
    ///
    /// This function will attempt to re-use old NodeIds if they exist.
    ///
    pub fn insert_new_node(&mut self, new_node: N) -> NodeId {

        if let Some(new_node_id) = self.free_ids.pop() {

            //there's a "hole" in self.nodes at new_node_id.index
            //so we can re-use an existing NodeId
            self.nodes.push(Some(new_node));
            self.nodes.swap_remove(new_node_id.index);

            new_node_id
        } else {

            let new_node_index = self.nodes.len();
            self.nodes.push(Some(new_node));

            self.new_node_id(new_node_index)
        }

    }


    ///
    /// Removes a node from the tree and returns it.
    ///
    /// This function will save the given NodeId for later use.
    ///
    pub fn remove_node(&mut self, node_id: NodeId) -> N {

        if Some(&node_id) == self.root.as_ref() {
            self.root = None;
        }

        self.nodes.push(None);

        if let Some(node) = self.nodes.swap_remove(node_id.index) {
            self.free_ids.push(node_id);
            node
        } else {
            panic!(
                "CoreTree.remove_node: An invalid NodeId made it past id_tree's internal \
                checks. Please report this issue!"
            );
        }
    }

    ///
    /// Generates a new NodeId for this tree.
    ///
    pub fn new_node_id(&self, node_index: usize) -> NodeId {
        NodeId {
            tree_id: self.id,
            index: node_index,
        }
    }

    ///
    /// Checks to see if a NodeId is valid.
    ///
    /// Nothing should make it past this function. If there is a way for a NodeId to be invalid,
    /// it should be caught here.
    ///
    pub fn is_valid_node_id(&self, node_id: &NodeId) -> (bool, Option<NodeIdError>) {
        if node_id.tree_id != self.id {
            return (false, Some(NodeIdError::InvalidNodeIdForTree));
        }

        if node_id.index >= self.nodes.len() {
            panic!(
                "NodeId: {:?} is out of bounds. This is most likely a bug in id_tree. Please \
                report this issue!",
                node_id
            );
        }

        unsafe {
            if self.nodes.get_unchecked(node_id.index).is_none() {
                return (false, Some(NodeIdError::NodeIdNoLongerValid));
            }
        }

        (true, None)
    }

    ///
    /// Returns an immutable reference to the Node that node_id points to.
    ///
    /// This function should only be called after a node_id has been check by
    /// CoreTree.is_valid_node_id().
    ///
    pub fn get_unsafe(&self, node_id: &NodeId) -> &N {
        unsafe {
            self.nodes.get_unchecked(node_id.index).as_ref().expect(
                "CoreTree.get_unsafe: An invalid NodeId made it past id_tree's internal \
                    checks.  Please report this issue!",
            )
        }
    }

    ///
    /// Returns a mutable reference to the Node that node_id points to.
    ///
    /// This function should only be called after a node_id has been check by
    /// CoreTree.is_valid_node_id().
    ///
    pub fn get_mut_unsafe(&mut self, node_id: &NodeId) -> &mut N {
        unsafe {
            self.nodes.get_unchecked_mut(node_id.index).as_mut().expect(
                "CoreTree.get_mut_unsafe: An invalid NodeId made it past id_tree's internal \
                    checks.  Please report this issue!",
            )
        }
    }
}

//todo: test this stuff.
