use std::marker::PhantomData;
use snowflake::ProcessUniqueId;

use ::*;

///
/// A structure for dealing with the core data that defines a Tree.
///
/// Hierarchical decisions should not be made at this level.  This structure should only know
/// how to do the most basic of operations and should take care of the logic that deals with
/// keeping its internal data structures in sync with one another.
///
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
    /// Inserts a new node and returns a NodeId that points to it.
    ///
    /// This function will attempt to re-use old NodeIds if they exist.
    ///
    pub fn insert(&mut self, new_node: N) -> NodeId {

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
    pub fn remove(&mut self, node_id: NodeId) -> N {

        if Some(&node_id) == self.root.as_ref() {
            self.root = None;
        }

        self.nodes.push(None);

        if let Some(node) = self.nodes.swap_remove(node_id.index) {
            self.free_ids.push(node_id);
            node
        } else {
            panic!(
                "CoreTree.remove: An invalid NodeId made it past id_tree's internal checks. \
                Please report this issue!"
            );
        }
    }

    ///
    /// Sets the root of the `Tree`.
    ///
    pub fn set_root(&mut self, new_root: N) -> NodeId {
        let new_root_id = self.insert(new_root);
        self.root = Some(new_root_id.clone());
        new_root_id
    }

    ///
    /// Returns an (optional) immutable reference to the NodeId that points to this Tree's root.
    ///
    pub fn root(&self) -> Option<&NodeId> {
        self.root.as_ref()
    }

    ///
    /// Return a Result containing an immutable reference to the Node that node_id points to,
    /// or a NodeIdError if one occurred.
    ///
    pub fn get(&self, node_id: &NodeId) -> Result<&N, NodeIdError> {
        self.validate_node_id(node_id)?;
        Ok(self.get_unsafe(node_id))
    }

    ///
    /// Return a Result containing a mutable reference to the Node that node_id points to,
    /// or a NodeIdError if one occurred.
    ///
    pub fn get_mut(&mut self, node_id: &NodeId) -> Result<&mut N, NodeIdError> {
        self.validate_node_id(node_id)?;
        Ok(self.get_mut_unsafe(node_id))
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

    ///
    /// Generates a new NodeId for this tree.
    ///
    fn new_node_id(&self, node_index: usize) -> NodeId {
        NodeId {
            tree_id: self.id,
            index: node_index,
        }
    }

    /// Deprecated...
    ///
    /// Checks to see if a NodeId is valid.
    ///
    /// Nothing should make it past this function. If there is a way for a NodeId to be invalid,
    /// it should be caught here.
    ///
    /// Deprecated...
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
    /// Checks to see if a NodeId is valid.
    ///
    /// Nothing should make it past this function. If there is a way for a NodeId to be invalid,
    /// it should be caught here.
    ///
    pub fn validate_node_id(&self, node_id: &NodeId) -> Result<(), NodeIdError> {
        if node_id.tree_id != self.id {
            return Err(NodeIdError::InvalidNodeIdForTree);
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
                return Err(NodeIdError::NodeIdNoLongerValid);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod core_tree_tests {
    use super::*;

    fn new_tree() -> CoreTree<VecNode<i32>, i32> {
        let root = Some(VecNode::new(1));
        let node_capacity = 2usize;
        let swap_capacity = 3usize;
        CoreTree::new(root, node_capacity, swap_capacity)
    }

    #[test]
    fn test_new() {
        let root = Some(VecNode::new(1));
        let node_capacity = 2usize;
        let swap_capacity = 3usize;

        let tree = CoreTree::new(root, node_capacity, swap_capacity);

        assert!(tree.root.is_some());

        let root_id = tree.root.as_ref().unwrap();

        assert_eq!(tree.get(root_id).unwrap().data(), &1);
        assert_eq!(tree.nodes.capacity(), node_capacity);
        assert_eq!(tree.free_ids.capacity(), swap_capacity);
    }

    #[test]
    fn test_insert() {
        let mut tree = new_tree();
        let four = 4;
        let node_id = tree.insert(VecNode::new(four));

        assert!(tree.get(&node_id).is_ok());
        assert_eq!(tree.get(&node_id).unwrap().data(), &four);
    }

    #[test]
    fn test_remove() {
        let mut tree = new_tree();
        let four = 4;
        let node_id = tree.insert(VecNode::new(four));

        assert!(tree.get(&node_id).is_ok());
        assert_eq!(tree.get(&node_id).unwrap().data(), &four);

        let node = tree.remove(node_id);
        assert_eq!(node.data(), &four);
    }

    #[test]
    #[should_panic]
    fn test_remove_twice() {
        let mut tree = new_tree();
        let four = 4;
        let node_id = tree.insert(VecNode::new(four));

        assert!(tree.get(&node_id).is_ok());
        assert_eq!(tree.get(&node_id).unwrap().data(), &four);

        //save it for later
        let node_id_clone = node_id.clone();

        let node = tree.remove(node_id);
        assert_eq!(node.data(), &four);

        //this should panic
        let _ = tree.remove(node_id_clone);
    }

    #[test]
    fn test_set_root() {
        let one = 1;
        let node_capacity = 2usize;
        let swap_capacity = 3usize;

        let mut tree = CoreTree::new(None, node_capacity, swap_capacity);

        assert!(tree.root.is_none());

        let root_id = tree.set_root(VecNode::new(one));

        assert!(tree.root.is_some());
        assert_eq!(tree.get(&root_id).unwrap().data(), &one);
    }

    #[test]
    fn test_root() {
        let mut tree = new_tree();
        assert!(tree.root.is_some());

        {
            let root_id = tree.root();
            assert!(root_id.is_some());
            assert_eq!(root_id, tree.root.as_ref());
        }

        let new_root_id = tree.set_root(VecNode::new(10));
        assert_eq!(Some(new_root_id).as_ref(), tree.root());
    }

    #[test]
    fn test_get() {
        let mut tree = new_tree();

        let four = 4;
        let node_id = tree.insert(VecNode::new(four));

        let node_result = tree.get(&node_id);
        assert!(node_result.is_ok());

        let node = node_result.unwrap();
        assert_eq!(node.data(), &four);
    }

    #[test]
    fn test_get_mut() {
        let mut tree = new_tree();

        let four = 4;
        let node_id = tree.insert(VecNode::new(four));

        let node_result = tree.get_mut(&node_id);
        assert!(node_result.is_ok());

        let node = node_result.unwrap();
        assert_eq!(node.data(), &four);

        let five = 5;
        let four_again = node.replace_data(five);
        assert_eq!(four_again, four);
        assert_eq!(node.data(), &five);
    }

    #[test]
    fn test_get_unsafe() {
        let mut tree = new_tree();

        let four = 4;
        let node_id = tree.insert(VecNode::new(four));

        let node = tree.get_unsafe(&node_id);
        assert_eq!(node.data(), &four);
    }

    #[test]
    #[should_panic]
    fn test_get_unsafe_after_removed() {
        let mut tree = new_tree();
        let four = 4;
        let node_id = tree.insert(VecNode::new(four));

        {
            let node_ref = tree.get_unsafe(&node_id);
            assert_eq!(node_ref.data(), &four);
        }

        //save it for later
        let node_id_clone = node_id.clone();

        let node = tree.remove(node_id);
        assert_eq!(node.data(), &four);

        //this should panic
        let _ = tree.get_unsafe(&node_id_clone);
    }

    #[test]
    fn test_get_mut_unsafe() {
        let mut tree = new_tree();

        let four = 4;
        let node_id = tree.insert(VecNode::new(four));

        let node = tree.get_mut_unsafe(&node_id);
        assert_eq!(node.data(), &four);

        let five = 5;
        let four_again = node.replace_data(five);
        assert_eq!(four_again, four);
        assert_eq!(node.data(), &five);
    }

    #[test]
    #[should_panic]
    fn test_get_mut_unsafe_after_removed() {
        let mut tree = new_tree();
        let four = 4;
        let node_id = tree.insert(VecNode::new(four));

        {
            let node_ref = tree.get_mut_unsafe(&node_id);
            assert_eq!(node_ref.data(), &four);
        }

        //save it for later
        let node_id_clone = node_id.clone();

        let node = tree.remove(node_id);
        assert_eq!(node.data(), &four);

        //this should panic
        let _ = tree.get_mut_unsafe(&node_id_clone);
    }

    #[test]
    fn test_new_node_id() {
        let tree = new_tree();

        // the index actually doesn't matter because we just care about the tree's id.
        let new_node_id = tree.new_node_id(0);

        assert_eq!(new_node_id.tree_id, tree.id);
    }

    #[test]
    fn test_is_valid_node_id() {
        let mut tree = new_tree();
        let node_id = tree.insert(VecNode::new(2));

        let (is_valid, error) = tree.is_valid_node_id(&node_id);
        assert!(is_valid);
        assert!(error.is_none());
    }

    #[test]
    fn test_is_valid_node_id_wrong_tree() {
        let tree_a = new_tree();
        let tree_b = new_tree();

        // the index actually doesn't matter because this is for the wrong tree.
        let node_id_a = tree_a.new_node_id(0);

        let (is_valid, error) = tree_b.is_valid_node_id(&node_id_a);
        assert!(!is_valid);
        assert!(error.is_some());
        assert_eq!(error.unwrap(), NodeIdError::InvalidNodeIdForTree);
    }

    #[test]
    fn test_is_valid_node_id_old_id() {
        let mut tree = new_tree();
        let node_id = tree.insert(VecNode::new(2));

        //save it for later
        let node_id_clone = node_id.clone();

        let node = tree.remove(node_id);

        let (is_valid, error) = tree.is_valid_node_id(&node_id_clone);
        assert!(!is_valid);
        assert!(error.is_some());
        assert_eq!(error.unwrap(), NodeIdError::NodeIdNoLongerValid);
    }

    #[test]
    fn test_validate_node_id() {
        let mut tree = new_tree();
        let node_id = tree.insert(VecNode::new(2));

        let result = tree.validate_node_id(&node_id);
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), ());
    }

    #[test]
    fn test_validate_node_id_wrong_tree() {
        let tree_a = new_tree();
        let tree_b = new_tree();

        // the index actually doesn't matter because this is for the wrong tree.
        let node_id_a = tree_a.new_node_id(0);

        let result = tree_b.validate_node_id(&node_id_a);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), NodeIdError::InvalidNodeIdForTree);
    }

    #[test]
    fn test_validate_node_id_old_id() {
        let mut tree = new_tree();
        let node_id = tree.insert(VecNode::new(2));

        //save it for later
        let node_id_clone = node_id.clone();

        let node = tree.remove(node_id);

        let result = tree.validate_node_id(&node_id_clone);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), NodeIdError::NodeIdNoLongerValid);
    }
}
