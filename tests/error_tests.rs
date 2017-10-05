extern crate id_tree;

use id_tree::NodeIdError::*;
use id_tree::*;
use id_tree::VecTreeBuilder;
use id_tree::VecTree;
use id_tree::RemoveBehavior::*;
use id_tree::MoveBehavior::*;
use id_tree::InsertBehavior::*;
use id_tree::SwapBehavior::*;

#[test]
fn test_insert_with_old_node_id() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id = tree_a.insert(VecNode::new(1), AsRoot).ok().unwrap();
    let root_id_copy = root_id.clone(); //save it for later

    let _ = tree_a.remove(root_id, DropChildren);

    //inserting under a node that is no longer in this tree.
    let result = tree_a.insert(VecNode::new(2), UnderNode(&root_id_copy));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn test_insert_into_wrong_tree() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id = tree_a.insert(VecNode::new(1), AsRoot).ok().unwrap();

    //inserting under a node that is in a different tree
    let result = tree_b.insert(VecNode::new(2), UnderNode(&root_id));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn test_get_node_with_old_node_id() {
    let mut tree: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_copy = root_id.clone();

    let _ = tree.remove(root_id, DropChildren);

    let root = tree.get(&root_id_copy);

    assert!(root.is_err());
    assert_eq!(root.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn test_get_node_from_wrong_tree() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();
    let tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_a = VecNode::new(1);
    let root_node_id_a = tree_a.insert(root_node_a, AsRoot).unwrap();

    let root_node_a = tree_a.get(&root_node_id_a);
    let root_node_b = tree_b.get(&root_node_id_a); //note use of wrong tree

    assert!(root_node_a.is_ok());
    assert!(root_node_b.is_err());
    assert_eq!(root_node_b.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn test_get_mut_node_with_old_node_id() {
    let mut tree: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_copy = root_id.clone();

    let _ = tree.remove(root_id, DropChildren);

    let root = tree.get_mut(&root_id_copy);

    assert!(root.is_err());
    assert_eq!(root.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn test_get_mut_node_from_wrong_tree() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_a = VecNode::new(1);
    let root_node_id_a = tree_a.insert(root_node_a, AsRoot).unwrap();

    let root_node_a = tree_a.get_mut(&root_node_id_a);
    let root_node_b = tree_b.get_mut(&root_node_id_a); //note use of wrong tree

    assert!(root_node_a.is_ok());
    assert!(root_node_b.is_err());
    assert_eq!(root_node_b.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn test_remove_lift_children_old_node_id() {
    let mut tree: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node = VecNode::new(1);

    let root_id = tree.insert(root_node, AsRoot).ok().unwrap();
    let root_id_copy = root_id.clone(); // this is essential to getting the Result::Err()

    let root_node = tree.remove(root_id, LiftChildren);
    assert!(root_node.is_ok());

    // removing a node that has already been removed
    let result = tree.remove(root_id_copy, LiftChildren);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn test_remove_orphan_children_old_node_id() {
    let mut tree: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node = VecNode::new(1);

    let root_id = tree.insert(root_node, AsRoot).ok().unwrap();
    let root_id_copy = root_id.clone(); // this is essential to getting the Result::Err()

    let root_node = tree.remove(root_id, OrphanChildren);
    assert!(root_node.is_ok());

    // removing a node that has already been removed
    let result = tree.remove(root_id_copy, OrphanChildren);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn test_remove_drop_children_old_node_id() {
    let mut tree: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node = VecNode::new(1);

    let root_id = tree.insert(root_node, AsRoot).ok().unwrap();
    let root_id_copy = root_id.clone(); // this is essential to getting the Result::Err()

    let root_node = tree.remove(root_id, DropChildren);
    assert!(root_node.is_ok());

    // removing a node that has already been removed.
    let result = tree.remove(root_id_copy, DropChildren);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn test_remove_lift_children_from_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_id_a = tree_a.insert(VecNode::new(1), AsRoot).unwrap();

    // note use of wrong tree
    let root_node_b = tree_b.remove(root_node_id_a, LiftChildren);

    assert!(root_node_b.is_err());
    assert_eq!(root_node_b.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn test_remove_orphan_children_from_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_id_a = tree_a.insert(VecNode::new(1), AsRoot).unwrap();

    // note use of wrong tree
    let root_node_b = tree_b.remove(root_node_id_a, OrphanChildren);

    assert!(root_node_b.is_err());
    assert_eq!(root_node_b.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn test_remove_drop_children_from_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_id_a = tree_a.insert(VecNode::new(1), AsRoot).unwrap();

    // note use of wrong tree
    let root_node_b = tree_b.remove(root_node_id_a, DropChildren);

    assert!(root_node_b.is_err());
    assert_eq!(root_node_b.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn test_move_node_to_root_with_old_node_id() {
    let mut tree = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let child_id = tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    let child_id_copy = child_id.clone(); //save it for later

    let _ = tree.remove(child_id, DropChildren);

    // moving node that has already been removed.
    let result = tree.move_node(&child_id_copy, ToRoot);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn test_move_node_to_root_in_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b = VecTreeBuilder::new().build();

    let _ = tree_a.insert(VecNode::new(1), AsRoot).unwrap();

    let root_id_b = tree_b.insert(VecNode::new(2), AsRoot).unwrap();

    // moving node from another tree.
    let result = tree_a.move_node(&root_id_b, ToRoot);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn test_move_node_into_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b = VecTreeBuilder::new().build();
    
    let root_id_a = tree_a.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_b = tree_b.insert(VecNode::new(2), AsRoot).unwrap();

    // moving child from correct tree to parent from the wrong tree
    let result = tree_a.move_node(&root_id_a, ToParent(&root_id_b));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn test_move_node_from_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b = VecTreeBuilder::new().build();

    let root_id_a = tree_a.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_b = tree_b.insert(VecNode::new(1), AsRoot).unwrap();

    // moving child from wrong tree to parent in correct tree.
    let result = tree_a.move_node(&root_id_b, ToParent(&root_id_a));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn test_sort_by_invalid_id() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_a = VecNode::new(1);
    let _ = tree_a.insert(root_node_a, AsRoot).unwrap();

    let root_node_b = VecNode::new(1);
    let root_node_id_b = tree_b.insert(root_node_b, AsRoot).unwrap();

    let result = tree_a.sort_children_by(&root_node_id_b, |a, b| a.data().cmp(b.data()));
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, InvalidNodeIdForTree);
}

#[test]
fn test_sort_by_data_invalid_id() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_a = VecNode::new(1);
    let _ = tree_a.insert(root_node_a, AsRoot).unwrap();

    let root_node_b = VecNode::new(1);
    let root_node_id_b = tree_b.insert(root_node_b, AsRoot).unwrap();

    let result = tree_a.sort_children_by_data(&root_node_id_b);
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, InvalidNodeIdForTree);
}

#[test]
fn test_sort_by_key_invalid_id() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_a = VecNode::new(1);
    let _ = tree_a.insert(root_node_a, AsRoot).unwrap();

    let root_node_b = VecNode::new(1);
    let root_node_id_b = tree_b.insert(root_node_b, AsRoot).unwrap();

    let result = tree_a.sort_children_by_key(&root_node_id_b, |x| *x.data());
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, InvalidNodeIdForTree);
}

#[test]
fn test_swap_sub_trees_of_different_trees() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_a = VecNode::new(1);
    let root_node_id_a = tree_a.insert(root_node_a, AsRoot).unwrap();

    let root_node_b = VecNode::new(1);
    let root_node_id_b = tree_b.insert(root_node_b, AsRoot).unwrap();

    // note use of invalid child
    let result = tree_a.swap_nodes(&root_node_id_b, &root_node_id_a, TakeChildren);
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error, InvalidNodeIdForTree);
}

#[test]
fn test_ancestors_different_trees() {
    let mut a = VecTree::new();
    let b = VecTree::<i32>::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();

    // note usage of `b` instead of `a`
    let ancestors = b.ancestors(&root_id);

    assert!(ancestors.is_err());
    let error = ancestors.err().unwrap();
    assert_eq!(error, InvalidNodeIdForTree);
}

#[test]
fn test_ancestors_old_id() {
    let mut a = VecTree::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    let _ = a.remove(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let ancestors = a.ancestors(&root_id_clone);

    assert!(ancestors.is_err());
    let error = ancestors.err().unwrap();
    assert_eq!(error, NodeIdNoLongerValid);
}

//#[test]
//fn test_ancestor_ids_different_trees() {
//    let mut a = VecTree::new();
//    let b = VecTree::<i32>::new();
//
//    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
//
//    // note usage of `b` instead of `a`
//    let ancestors = b.ancestor_ids(&root_id);
//
//    assert!(ancestors.is_err());
//    let error = ancestors.err().unwrap();
//    assert_eq!(error, InvalidNodeIdForTree);
//}

//#[test]
//fn test_ancestor_ids_old_id() {
//    let mut a = VecTree::new();
//
//    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
//    // `.clone()` required to get this error
//    let root_id_clone = root_id.clone();
//    let _ = a.remove(root_id, DropChildren).unwrap();
//
//    // note usage of cloned `NodeId`
//    let ancestors = a.ancestor_ids(&root_id_clone);
//
//    assert!(ancestors.is_err());
//    let error = ancestors.err().unwrap();
//    assert_eq!(error, NodeIdNoLongerValid);
//}

//#[test]
//fn test_children_different_trees() {
//    let mut a = VecTree::new();
//    let b = VecTree::<i32>::new();
//
//    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
//
//    // note usage of `b` instead of `a`
//    let ancestors = b.children(&root_id);
//
//    assert!(ancestors.is_err());
//    let error = ancestors.err().unwrap();
//    assert_eq!(error, InvalidNodeIdForTree);
//}

//#[test]
//fn test_children_old_id() {
//    let mut a = VecTree::new();
//
//    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
//    // `.clone()` required to get this error
//    let root_id_clone = root_id.clone();
//    let _ = a.remove(root_id, DropChildren).unwrap();
//
//    // note usage of cloned `NodeId`
//    let ancestors = a.children(&root_id_clone);
//
//    assert!(ancestors.is_err());
//    let error = ancestors.err().unwrap();
//    assert_eq!(error, NodeIdNoLongerValid);
//}

//#[test]
//fn test_children_ids_different_trees() {
//    let mut a = VecTree::new();
//    let b = VecTree::<i32>::new();
//
//    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
//
//    // note usage of `b` instead of `a`
//    let ancestors = b.children_ids(&root_id);
//
//    assert!(ancestors.is_err());
//    let error = ancestors.err().unwrap();
//    assert_eq!(error, InvalidNodeIdForTree);
//}
//
//#[test]
//fn test_children_ids_old_id() {
//    let mut a = VecTree::new();
//
//    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
//    // `.clone()` required to get this error
//    let root_id_clone = root_id.clone();
//    let _ = a.remove(root_id, DropChildren).unwrap();
//
//    // note usage of cloned `NodeId`
//    let ancestors = a.children_ids(&root_id_clone);
//
//    assert!(ancestors.is_err());
//    let error = ancestors.err().unwrap();
//    assert_eq!(error, NodeIdNoLongerValid);
//}
