use std::slice::Iter;
use std::vec::IntoIter;
use std::collections::VecDeque;

use Tree;
use Node;
use NodeId;
use tree::IteratorNew;

///
/// An Iterator over the ancestors of a `Node`.
///
/// Iterates over the ancestor `Node`s of a given `Node` in the `Tree`.  Each call to `next` will
/// return an immutable reference to the next `Node` up the `Tree`.
///
pub struct Ancestors<'a, T: 'a> {
    tree: &'a Tree<T>,
    node_id: Option<NodeId>,
}

impl<'a, T> IteratorNew<'a, T, Ancestors<'a, T>> for Ancestors<'a, T> {
    fn new(tree: &'a Tree<T>, node_id: NodeId) -> Ancestors<'a, T> {
        Ancestors {
            tree: tree,
            node_id: Some(node_id),
        }
    }
}

impl<'a, T> Iterator for Ancestors<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<&'a Node<T>> {
        if let Some(current_id) = self.node_id.clone() {
            if let Some(parent_id) = self.tree.get_unsafe(&current_id).parent() {
                let parent = self.tree.get_unsafe(&parent_id);
                self.node_id = Some(parent_id.clone());
                return Some(parent);
            } else {
                self.node_id = None;
            }
        }
        None
    }
}

///
/// An Iterator over the ancestors of a `Node`.
///
/// Iterates over `NodeId`s instead of over the `Node`s themselves.
///
pub struct AncestorIds<'a, T: 'a> {
    tree: &'a Tree<T>,
    node_id: Option<NodeId>,
}

impl<'a, T> IteratorNew<'a, T, AncestorIds<'a, T>> for AncestorIds<'a, T> {
    fn new(tree: &'a Tree<T>, node_id: NodeId) -> AncestorIds<'a, T> {
        AncestorIds {
            tree: tree,
            node_id: Some(node_id),
        }
    }
}

impl<'a, T> Iterator for AncestorIds<'a, T> {
    type Item = &'a NodeId;

    fn next(&mut self) -> Option<&'a NodeId> {
        if let Some(current_id) = self.node_id.clone() {
            if let Some(parent_id) = self.tree.get_unsafe(&current_id).parent() {
                self.node_id = Some(parent_id.clone());
                return Some(parent_id);
            } else {
                self.node_id = None;
            }
        }
        None
    }
}

///
/// An Iterator over the children of a `Node`.
///
/// Iterates over the child `Node`s of a given `Node` in the `Tree`.  Each call to `next` will
/// return an immutable reference to the next child `Node`.
///
pub struct Children<'a, T: 'a> {
    tree: &'a Tree<T>,
    child_ids: Iter<'a, NodeId>,
}

impl<'a, T> IteratorNew<'a, T, Children<'a, T>> for Children<'a, T> {
    fn new(tree: &'a Tree<T>, node_id: NodeId) -> Children<'a, T> {
        Children {
            tree: tree,
            child_ids: tree.get_unsafe(&node_id).children().as_slice().iter(),
        }
    }
}

impl<'a, T> Iterator for Children<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<&'a Node<T>> {
        if let Some(ref next_child_id) = self.child_ids.next() {
            return Some(self.tree.get_unsafe(next_child_id));
        }
        None
    }
}

///
/// An Iterator over the children of a `Node`.
///
/// Iterates over `NodeId`s instead of over the `Node`s themselves.
///
pub struct ChildrenIds<'a> {
    child_ids: Iter<'a, NodeId>,
}

impl<'a, T> IteratorNew<'a, T, ChildrenIds<'a>> for ChildrenIds<'a> {
    fn new(tree: &'a Tree<T>, node_id: NodeId) -> ChildrenIds<'a> {
        ChildrenIds { child_ids: tree.get_unsafe(&node_id).children().as_slice().iter() }
    }
}

impl<'a> Iterator for ChildrenIds<'a> {
    type Item = &'a NodeId;

    fn next(&mut self) -> Option<&'a NodeId> {
        self.child_ids.next()
    }
}

///
/// An Iterator over the sub-tree relative to a given `Node`.
///
/// Iterates over all of the `Node`s in the sub-tree of a given `Node` in the `Tree`.  Each call to
/// `next` will return an immutable reference to the next `Node` in Pre-Order Traversal order.
///
pub struct PreOrderTraversal<'a, T: 'a> {
    tree: &'a Tree<T>,
    data: VecDeque<NodeId>,
}

impl<'a, T> IteratorNew<'a, T, PreOrderTraversal<'a, T>> for PreOrderTraversal<'a, T> {
    fn new(tree: &'a Tree<T>, node_id: NodeId) -> PreOrderTraversal<T> {

        // over allocating, but all at once instead of re-sizing and re-allocating as we go
        let mut data = VecDeque::with_capacity(tree.nodes.capacity());

        data.push_front(node_id);

        PreOrderTraversal {
            tree: tree,
            data: data,
        }
    }
}

impl<'a, T> Iterator for PreOrderTraversal<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<&'a Node<T>> {
        let id = self.data.pop_front();

        if let Some(ref node_id) = id {
            let node_ref = self.tree.get_unsafe(node_id);

            // prepend child_ids
            for child_id in node_ref.children().iter().rev() {
                self.data.push_front(child_id.clone());
            }

            return Some(node_ref);
        }
        None
    }
}

///
/// An Iterator over the sub-tree relative to a given `Node`.
///
/// Iterates over all of the `Node`s in the sub-tree of a given `Node` in the `Tree`.  Each call to
/// `next` will return an immutable reference to the next `Node` in Post-Order Traversal order.
///
pub struct PostOrderTraversal<'a, T: 'a> {
    tree: &'a Tree<T>,
    ids: IntoIter<NodeId>,
}

impl<'a, T> PostOrderTraversal<'a, T> {
    fn process_nodes(starting_id: NodeId, tree: &Tree<T>, ids: &mut Vec<NodeId>) {
        let node = tree.get_unsafe(&starting_id);

        for child_id in node.children() {
            PostOrderTraversal::process_nodes(child_id.clone(), tree, ids);
        }

        ids.push(starting_id);
    }
}

impl<'a, T> IteratorNew<'a, T, PostOrderTraversal<'a, T>> for PostOrderTraversal<'a, T> {
    fn new(tree: &'a Tree<T>, node_id: NodeId) -> PostOrderTraversal<T> {

        // over allocating, but all at once instead of re-sizing and re-allocating as we go
        let mut ids = Vec::with_capacity(tree.nodes.capacity());

        PostOrderTraversal::process_nodes(node_id, tree, &mut ids);

        PostOrderTraversal {
            tree: tree,
            ids: ids.into_iter(),
        }
    }
}

impl<'a, T> Iterator for PostOrderTraversal<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<&'a Node<T>> {
        let id = self.ids.next();

        if let Some(ref node_id) = id {
            return Some(self.tree.get_unsafe(node_id));
        }

        None
    }
}

///
/// An Iterator over the sub-tree relative to a given `Node`.
///
/// Iterates over all of the `Node`s in the sub-tree of a given `Node` in the `Tree`.  Each call to
/// `next` will return an immutable reference to the next `Node` in Level-Order Traversal order.
///
pub struct LevelOrderTraversal<'a, T: 'a> {
    tree: &'a Tree<T>,
    data: VecDeque<NodeId>,
}

impl<'a, T> IteratorNew<'a, T, LevelOrderTraversal<'a, T>> for LevelOrderTraversal<'a, T> {
    fn new(tree: &'a Tree<T>, node_id: NodeId) -> LevelOrderTraversal<T> {

        // over allocating, but all at once instead of re-sizing and re-allocating as we go
        let mut data = VecDeque::with_capacity(tree.nodes.capacity());

        data.push_back(node_id);

        LevelOrderTraversal {
            tree: tree,
            data: data,
        }
    }
}

impl<'a, T> Iterator for LevelOrderTraversal<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<&'a Node<T>> {
        let id = self.data.pop_front();

        if let Some(ref node_id_ref) = id {

            let node_ref = self.tree.get_unsafe(node_id_ref);

            for child_id in node_ref.children() {
                self.data.push_back(child_id.clone());
            }

            return Some(node_ref);
        }

        None
    }
}

#[cfg(test)]
mod tests {

    use Tree;
    use Node;
    use InsertBehavior::*;

    #[test]
    fn test_ancestors() {
        let mut tree = Tree::new();

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
    fn test_ancestor_ids() {
        let mut tree = Tree::new();

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
    fn test_children() {
        let mut tree = Tree::new();

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
    fn test_children_ids() {
        let mut tree = Tree::new();

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
    fn test_pre_order_traversal() {
        let mut tree = Tree::new();

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
    fn test_post_order_traversal() {
        let mut tree = Tree::new();

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
    fn test_level_order_traversal() {
        let mut tree = Tree::new();

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
