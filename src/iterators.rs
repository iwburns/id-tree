use std::slice::Iter;
use std::marker::PhantomData;
use std::vec::IntoIter;
use std::collections::VecDeque;

use tree::*;
use node::*;
use NodeId;

//todo: tests for OptTree versions of iterators.

///
/// An Iterator over the ancestors of a `Node`.
///
/// Iterates over the ancestor `Node`s of a given `Node` in the `Tree`.  Each call to `next` will
/// return an immutable reference to the next `Node` up the `Tree`.
///
///
pub struct Ancestors<'a, T: 'a, D>
where
    T: Tree<'a, D>,
{
    tree: &'a T,
    node_id: Option<NodeId>,
    phantom: PhantomData<D>,
}

impl<'a, T, D> Ancestors<'a, T, D>
where
    T: Tree<'a, D>,
{
    pub(crate) fn new<'f>(tree: &'f T, node_id: NodeId) -> Ancestors<'a, T, D>
    where
        'f: 'a,
    {
        Ancestors {
            tree: tree,
            node_id: Some(node_id),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a, D: 'a> Iterator for Ancestors<'a, T, D>
where
    T: Tree<'a, D>,
    T::NodeType: 'a,
{
    type Item = &'a T::NodeType;

    fn next(&mut self) -> Option<Self::Item> {
        let parent_id = self.node_id.as_ref().and_then(|current_id| {
            let current = unsafe { self.tree.get_unchecked(current_id) };
            current.parent()
        });

        let next = parent_id.map(|id| unsafe { self.tree.get_unchecked(id) });

        self.node_id = parent_id.cloned();

        next
    }
}

///
/// An Iterator over the ancestors of a `Node`.
///
/// Iterates over `NodeId`s instead of over the `Node`s themselves.
///
pub struct AncestorIds<'a, T: 'a, D>
where
    T: Tree<'a, D>,
{
    tree: &'a T,
    node_id: Option<NodeId>,
    phantom: PhantomData<D>,
}

impl<'a, T, D> AncestorIds<'a, T, D>
where
    T: Tree<'a, D>,
{
    pub(crate) fn new<'f>(tree: &'f T, node_id: NodeId) -> AncestorIds<'a, T, D>
    where
        'f: 'a,
    {
        AncestorIds {
            tree: tree,
            node_id: Some(node_id),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: 'a, D: 'a> Iterator for AncestorIds<'a, T, D>
where
    T: Tree<'a, D>,
    T::NodeType: 'a,
{
    type Item = &'a NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.node_id.as_ref().and_then(|current_id| {
            let current = unsafe { self.tree.get_unchecked(current_id) };
            current.parent()
        });

        self.node_id = next.cloned();

        next
    }
}

///
/// An Iterator over the children of a `VecNode`.
///
/// Iterates over the child `VecNode`s of a given `VecNode` in the `VecTree`.  Each call to `next`
/// will return an immutable reference to the next child `VecNode`.
///
pub struct VecChildren<'a, D: 'a> {
    tree: &'a VecTree<'a, D>,
    child_ids: Iter<'a, NodeId>,
}

impl<'a, D> VecChildren<'a, D> {
    pub(crate) fn new(tree: &'a VecTree<D>, node_id: NodeId) -> VecChildren<'a, D> {
        VecChildren {
            tree,
            child_ids: tree.core_tree()
                .get_unsafe(&node_id)
                .children()
                .as_slice()
                .iter(),
        }
    }
}

impl<'a, D> Iterator for VecChildren<'a, D> {
    type Item = &'a VecNode<D>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_child_id) = self.child_ids.next() {
            return Some(self.tree.core_tree().get_unsafe(next_child_id));
        }
        None
    }
}

///
/// An Iterator over the children of an `OptNode`.
///
/// Iterates of the child `OptNode`s of a given `OptNode` in the `OptTree`.  Each call to `next`
/// will return an immutable reference to the next child `OptNode`.
///
pub struct OptChildren<'a, T: 'a> {
    tree: &'a OptTree<'a, T>,
    node_id: Option<NodeId>,
}

impl<'a, T> OptChildren<'a, T> {
    pub(crate) fn new(tree: &'a OptTree<T>, node_id: NodeId) -> OptChildren<'a, T> {
        let base_node = unsafe { tree.get_unchecked(&node_id) };
        OptChildren {
            tree,
            node_id: base_node.first_child().cloned(),
        }
    }
}

impl<'a, T> Iterator for OptChildren<'a, T> {
    type Item = &'a OptNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

///
/// An Iterator over the children of a `VecNode`.
///
/// Iterates over `NodeId`s instead of over the `VecNode`s themselves.
///
pub struct VecChildrenIds<'a> {
    child_ids: Iter<'a, NodeId>,
}

impl<'a> VecChildrenIds<'a> {
    pub(crate) fn new<T>(tree: &'a VecTree<T>, node_id: NodeId) -> VecChildrenIds<'a> {
        VecChildrenIds {
            child_ids: tree.core_tree()
                .get_unsafe(&node_id)
                .children()
                .as_slice()
                .iter(),
        }
    }
}

impl<'a> Iterator for VecChildrenIds<'a> {
    type Item = &'a NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        self.child_ids.next()
    }
}

///
/// An Iterator over the children of an `OptNode`.
///
/// Iterates over `NodeId`s instead of over the `OptNode`s themselves.
///
pub struct OptChildrenIds<'a, T: 'a> {
    tree: &'a OptTree<'a, T>,
    node_id: Option<NodeId>,
}

impl<'a, T> OptChildrenIds<'a, T> {
    pub(crate) fn new(tree: &'a OptTree<T>, node_id: NodeId) -> OptChildrenIds<'a, T> {
        let base_node = unsafe { tree.get_unchecked(&node_id) };
        OptChildrenIds {
            tree,
            node_id: base_node.first_child().cloned(),
        }
    }
}

impl<'a, T> Iterator for OptChildrenIds<'a, T> {
    type Item = &'a NodeId;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

///
/// An Iterator over the sub-tree relative to a given `VecNode`.
///
/// Iterates over all of the `VecNode`s in the sub-tree of a given `VecNode` in the `VecTree`.
/// Each call to `next` will return an immutable reference to the next `VecNode` in Pre-Order
/// Traversal order.
///
pub struct VecPreOrderTraversal<'a, T: 'a> {
    tree: &'a VecTree<'a, T>,
    data: VecDeque<NodeId>,
}

impl<'a, T> VecPreOrderTraversal<'a, T> {
    pub(crate) fn new(tree: &'a VecTree<'a, T>, node_id: NodeId) -> VecPreOrderTraversal<'a, T> {

        // over allocating, but all at once instead of re-sizing and re-allocating as we go
        let mut data = VecDeque::with_capacity(tree.core_tree().nodes.capacity());

        data.push_front(node_id);

        VecPreOrderTraversal { tree, data }
    }
}

impl<'a, T> Iterator for VecPreOrderTraversal<'a, T> {
    type Item = &'a VecNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.data.pop_front();

        if let Some(ref node_id) = id {
            let node_ref = self.tree.core_tree().get_unsafe(node_id);

            // prepend child_ids
            for child_id in node_ref.children().iter().rev() {
                self.data.push_front(child_id.clone());
            }

            return Some(node_ref);
        }
        None
    }
}

//todo: add an OptPreOrderTraversal iterator

///
/// An Iterator over the sub-tree relative to a given `VecNode`.
///
/// Iterates over all of the `VecNode`s in the sub-tree of a given `VecNode` in the `VecTree`.
/// Each call to `next` will return an immutable reference to the next `VecNode` in Post-Order
/// Traversal order.
///
pub struct VecPostOrderTraversal<'a, T: 'a> {
    tree: &'a VecTree<'a, T>,
    ids: IntoIter<NodeId>,
}

impl<'a, T> VecPostOrderTraversal<'a, T> {
    pub(crate) fn new(tree: &'a VecTree<'a, T>, node_id: NodeId) -> VecPostOrderTraversal<'a, T> {

        // over allocating, but all at once instead of re-sizing and re-allocating as we go
        let mut ids = Vec::with_capacity(tree.core_tree().nodes.capacity());

        VecPostOrderTraversal::process_nodes(node_id, tree, &mut ids);

        VecPostOrderTraversal {
            tree,
            ids: ids.into_iter(),
        }
    }

    fn process_nodes(starting_id: NodeId, tree: &VecTree<T>, ids: &mut Vec<NodeId>) {
        let node = tree.core_tree().get_unsafe(&starting_id);

        for child_id in node.children() {
            VecPostOrderTraversal::process_nodes(child_id.clone(), tree, ids);
        }

        ids.push(starting_id);
    }
}

impl<'a, T> Iterator for VecPostOrderTraversal<'a, T> {
    type Item = &'a VecNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.next();

        if let Some(ref node_id) = id {
            return Some(self.tree.core_tree().get_unsafe(node_id));
        }

        None
    }
}

//todo: add an OptPostOrderTraversal iterator

///
/// An Iterator over the sub-tree relative to a given `VecNode`.
///
/// Iterates over all of the `VecNode`s in the sub-tree of a given `VecNode` in the `VecTree`.
/// Each call to `next` will return an immutable reference to the next `VecNode` in Level-Order
/// Traversal order.
///
pub struct VecLevelOrderTraversal<'a, T: 'a> {
    tree: &'a VecTree<'a, T>,
    data: VecDeque<NodeId>,
}

impl<'a, T> VecLevelOrderTraversal<'a, T> {
    pub(crate) fn new(tree: &'a VecTree<'a, T>, node_id: NodeId) -> VecLevelOrderTraversal<T> {

        // over allocating, but all at once instead of re-sizing and re-allocating as we go
        let mut data = VecDeque::with_capacity(tree.core_tree().nodes.capacity());

        data.push_back(node_id);

        VecLevelOrderTraversal { tree, data }
    }
}

impl<'a, T> Iterator for VecLevelOrderTraversal<'a, T> {
    type Item = &'a VecNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.data.pop_front();

        if let Some(ref node_id_ref) = id {

            let node_ref = self.tree.core_tree().get_unsafe(node_id_ref);

            for child_id in node_ref.children() {
                self.data.push_back(child_id.clone());
            }

            return Some(node_ref);
        }

        None
    }
}

//todo: add an OptLevelOrderTraversal iterator

#[cfg(test)]
mod tests {
    use ::*;
    use behaviors::InsertBehavior::*;

    #[test]
    fn vec_ancestors() {
        let mut tree = VecTree::new();

        let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        let node_1 = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2 = tree.insert(Node::new(2), UnderNode(&node_1)).unwrap();
        let node_3 = tree.insert(Node::new(3), UnderNode(&node_1)).unwrap();

        let ancestors = tree.ancestors(&root_id).unwrap();
        assert_eq!(ancestors.count(), 0);

        let data = [0];
        for (index, node) in tree.ancestors(&node_1).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [1, 0];
        for (index, node) in tree.ancestors(&node_2).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [1, 0];
        for (index, node) in tree.ancestors(&node_3).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }
    }

    #[test]
    fn vec_ancestor_ids() {
        let mut tree = VecTree::new();

        let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        let node_1 = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2 = tree.insert(Node::new(2), UnderNode(&node_1)).unwrap();
        let node_3 = tree.insert(Node::new(3), UnderNode(&node_1)).unwrap();

        let ancestor_ids = tree.ancestor_ids(&root_id).unwrap();
        assert_eq!(ancestor_ids.count(), 0);

        let data = [0];
        for (index, node_id) in tree.ancestor_ids(&node_1).unwrap().enumerate() {
            assert_eq!(tree.get(node_id).unwrap().data(), &data[index]);
        }

        let data = [1, 0];
        for (index, node_id) in tree.ancestor_ids(&node_2).unwrap().enumerate() {
            assert_eq!(tree.get(node_id).unwrap().data(), &data[index]);
        }

        let data = [1, 0];
        for (index, node_id) in tree.ancestor_ids(&node_3).unwrap().enumerate() {
            assert_eq!(tree.get(node_id).unwrap().data(), &data[index]);
        }
    }

    #[test]
    fn vec_children() {
        let mut tree = VecTree::new();

        let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        let node_1 = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2 = tree.insert(Node::new(2), UnderNode(&node_1)).unwrap();
        let node_3 = tree.insert(Node::new(3), UnderNode(&node_1)).unwrap();

        let data = [1];
        for (index, node) in tree.children(&root_id).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [2, 3];
        for (index, node) in tree.children(&node_1).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let children = tree.children(&node_2).unwrap();
        assert_eq!(children.count(), 0);

        let children = tree.children(&node_3).unwrap();
        assert_eq!(children.count(), 0);
    }

    #[test]
    fn vec_children_ids() {
        let mut tree = VecTree::new();

        let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        let node_1 = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2 = tree.insert(Node::new(2), UnderNode(&node_1)).unwrap();
        let node_3 = tree.insert(Node::new(3), UnderNode(&node_1)).unwrap();

        let data = [1];
        for (index, node_id) in tree.children_ids(&root_id).unwrap().enumerate() {
            assert_eq!(tree.get(node_id).unwrap().data(), &data[index]);
        }

        let data = [2, 3];
        for (index, node_id) in tree.children_ids(&node_1).unwrap().enumerate() {
            assert_eq!(tree.get(node_id).unwrap().data(), &data[index]);
        }

        let children_ids = tree.children_ids(&node_2).unwrap();
        assert_eq!(children_ids.count(), 0);

        let children_ids = tree.children_ids(&node_3).unwrap();
        assert_eq!(children_ids.count(), 0);
    }

    #[test]
    fn vec_pre_order_traversal() {
        let mut tree = VecTree::new();

        //      0
        //     / \
        //    1   2
        //   /
        //  3
        let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        let node_1 = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2 = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
        let node_3 = tree.insert(Node::new(3), UnderNode(&node_1)).unwrap();

        let data = [0, 1, 3, 2];
        for (index, node) in tree.traverse_pre_order(&root_id).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [1, 3];
        for (index, node) in tree.traverse_pre_order(&node_1).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [2];
        for (index, node) in tree.traverse_pre_order(&node_2).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [3];
        for (index, node) in tree.traverse_pre_order(&node_3).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }
    }

    #[test]
    fn vec_post_order_traversal() {
        let mut tree = VecTree::new();

        //      0
        //     / \
        //    1   2
        //   /
        //  3
        let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        let node_1 = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2 = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
        let node_3 = tree.insert(Node::new(3), UnderNode(&node_1)).unwrap();

        let data = [3, 1, 2, 0];
        for (index, node) in tree.traverse_post_order(&root_id).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [3, 1];
        for (index, node) in tree.traverse_post_order(&node_1).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [2];
        for (index, node) in tree.traverse_post_order(&node_2).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [3];
        for (index, node) in tree.traverse_post_order(&node_3).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }
    }

    #[test]
    fn vec_level_order_traversal() {
        let mut tree = VecTree::new();

        //      0
        //     / \
        //    1   2
        //   /
        //  3
        let root_id = tree.insert(Node::new(0), AsRoot).unwrap();
        let node_1 = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
        let node_2 = tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
        let node_3 = tree.insert(Node::new(3), UnderNode(&node_1)).unwrap();

        let data = [0, 1, 2, 3];
        for (index, node) in tree.traverse_level_order(&root_id).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [1, 3];
        for (index, node) in tree.traverse_level_order(&node_1).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [2];
        for (index, node) in tree.traverse_level_order(&node_2).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }

        let data = [3];
        for (index, node) in tree.traverse_level_order(&node_3).unwrap().enumerate() {
            assert_eq!(node.data(), &data[index]);
        }
    }
}
