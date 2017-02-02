use super::Tree;
use super::Node;
use super::NodeId;
use super::IteratorNew;


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

use std::slice::Iter;

pub struct Children<'a, T: 'a> {
    tree: &'a Tree<T>,
    child_ids: Iter<'a, NodeId>,
}

impl<'a, T> IteratorNew<'a, T, Children<'a, T>> for Children<'a, T> {
    fn new(tree: &'a Tree<T>, node_id: NodeId) -> Children<'a, T> {
        Children {
            tree: tree,
            child_ids: tree.get_unsafe(&node_id).children().as_slice().iter()
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
}