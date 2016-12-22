extern crate id_tree;

use id_tree::NodeIdError;
use id_tree::Node;
use id_tree::TreeBuilder;
use id_tree::Tree;

#[test]
fn test_old_node_id() {
    let mut tree: Tree<i32> = TreeBuilder::new().build();

    let root_node = Node::new(1);

    let root_id = tree.set_root(root_node);
    let root_id_copy = root_id.clone(); // this is essential to getting the Result::Err()

    let root_node = tree.remove_node_orphan_children(root_id);
    assert!(root_node.is_ok());

    let root_node_again = tree.remove_node_orphan_children(root_id_copy);
    assert!(root_node_again.is_err());

    let error = root_node_again.err().unwrap();
    assert_eq!(error, NodeIdError::NodeIdNoLongerValid);
}

#[test]
fn test_get_node_from_other_tree() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_a = Node::new(1);
    let root_node_id_a = tree_a.set_root(root_node_a);

    let root_node_a = tree_a.get(&root_node_id_a);
    let root_node_b = tree_b.get(&root_node_id_a); //note use of wrong tree

    assert!(root_node_a.is_some());
    assert!(root_node_b.is_none());
}

#[test]
fn test_get_mut_node_from_other_tree() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_a = Node::new(1);
    let root_node_id_a = tree_a.set_root(root_node_a);

    let root_node_a = tree_a.get_mut(&root_node_id_a);
    let root_node_b = tree_b.get_mut(&root_node_id_a); //note use of wrong tree

    assert!(root_node_a.is_some());
    assert!(root_node_b.is_none());
}

#[test]
fn test_remove_node_lift_children_from_other_tree() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_id_a = tree_a.set_root(Node::new(1));

    let root_node_b = tree_b.remove_node_lift_children(root_node_id_a); //note use of wrong tree
    assert!(root_node_b.is_err());

    let error = root_node_b.err().unwrap();
    assert_eq!(error, NodeIdError::InvalidNodeIdForTree);
}

#[test]
fn test_remove_node_orphan_children_from_other_tree() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_id_a = tree_a.set_root(Node::new(1));

    let root_node_b = tree_b.remove_node_orphan_children(root_node_id_a); //note use of wrong tree
    assert!(root_node_b.is_err());

    let error = root_node_b.err().unwrap();
    assert_eq!(error, NodeIdError::InvalidNodeIdForTree);
}

#[test]
fn test_remove_node_remove_children_from_other_tree() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_id_a = tree_a.set_root(Node::new(1));

    let root_node_b = tree_b.remove_node_drop_children(root_node_id_a); //note use of wrong tree
    assert!(root_node_b.is_err());

    let error = root_node_b.err().unwrap();
    assert_eq!(error, NodeIdError::InvalidNodeIdForTree);
}

#[test]
fn test_move_node_into_other_tree() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_a = Node::new(1);
    let root_node_id_a = tree_a.set_root(root_node_a);

    let root_node_b = Node::new(1);
    let root_node_id_b = tree_b.set_root(root_node_b);

    let result = tree_a.move_node_to_parent(&root_node_id_a, &root_node_id_b); //note use of invalid parent
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, NodeIdError::InvalidNodeIdForTree);
}

#[test]
fn test_move_node_from_other_tree() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_a = Node::new(1);
    let root_node_id_a = tree_a.set_root(root_node_a);

    let root_node_b = Node::new(1);
    let root_node_id_b = tree_b.set_root(root_node_b);

    let result = tree_a.move_node_to_parent(&root_node_id_b, &root_node_id_a); //note use of invalid child
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, NodeIdError::InvalidNodeIdForTree);
}

#[test]
fn test_sort_by_invalid_id() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_a = Node::new(1);
    let _ = tree_a.set_root(root_node_a);

    let root_node_b = Node::new(1);
    let root_node_id_b = tree_b.set_root(root_node_b);

    let result = tree_a.sort_children_by(&root_node_id_b, |a, b| a.data().cmp(b.data()));
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, NodeIdError::InvalidNodeIdForTree);
}

#[test]
fn test_sort_by_data_invalid_id() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_a = Node::new(1);
    let _ = tree_a.set_root(root_node_a);

    let root_node_b = Node::new(1);
    let root_node_id_b = tree_b.set_root(root_node_b);

    let result = tree_a.sort_children_by_data(&root_node_id_b);
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, NodeIdError::InvalidNodeIdForTree);
}

#[test]
fn test_sort_by_key_invalid_id() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_a = Node::new(1);
    let _ = tree_a.set_root(root_node_a);

    let root_node_b = Node::new(1);
    let root_node_id_b = tree_b.set_root(root_node_b);

    let result = tree_a.sort_children_by_key(&root_node_id_b, |x| x.data().clone());
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, NodeIdError::InvalidNodeIdForTree);
}

#[test]
fn test_swap_sub_trees_of_different_trees() {
    let mut tree_a: Tree<i32> = TreeBuilder::new().build();
    let mut tree_b: Tree<i32> = TreeBuilder::new().build();

    let root_node_a = Node::new(1);
    let root_node_id_a = tree_a.set_root(root_node_a);

    let root_node_b = Node::new(1);
    let root_node_id_b = tree_b.set_root(root_node_b);

    let result = tree_a.swap_sub_tree(&root_node_id_b, &root_node_id_a); //note use of invalid child
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, NodeIdError::InvalidNodeIdForTree);
}
