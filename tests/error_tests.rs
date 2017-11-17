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
fn insert_with_old_node_id() {
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
fn insert_into_wrong_tree() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id = tree_a.insert(VecNode::new(1), AsRoot).ok().unwrap();

    //inserting under a node that is in a different tree
    let result = tree_b.insert(VecNode::new(2), UnderNode(&root_id));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn get_node_with_old_node_id() {
    let mut tree: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_copy = root_id.clone();

    let _ = tree.remove(root_id, DropChildren);

    let root = tree.get(&root_id_copy);

    assert!(root.is_err());
    assert_eq!(root.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn get_node_from_wrong_tree() {
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
fn get_mut_node_with_old_node_id() {
    let mut tree: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_copy = root_id.clone();

    let _ = tree.remove(root_id, DropChildren);

    let root = tree.get_mut(&root_id_copy);

    assert!(root.is_err());
    assert_eq!(root.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn get_mut_node_from_wrong_tree() {
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
fn remove_lift_children_old_node_id() {
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
fn remove_orphan_children_old_node_id() {
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
fn remove_drop_children_old_node_id() {
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
fn remove_lift_children_from_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_id_a = tree_a.insert(VecNode::new(1), AsRoot).unwrap();

    // note use of wrong tree
    let root_node_b = tree_b.remove(root_node_id_a, LiftChildren);

    assert!(root_node_b.is_err());
    assert_eq!(root_node_b.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn remove_orphan_children_from_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_id_a = tree_a.insert(VecNode::new(1), AsRoot).unwrap();

    // note use of wrong tree
    let root_node_b = tree_b.remove(root_node_id_a, OrphanChildren);

    assert!(root_node_b.is_err());
    assert_eq!(root_node_b.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn remove_drop_children_from_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_id_a = tree_a.insert(VecNode::new(1), AsRoot).unwrap();

    // note use of wrong tree
    let root_node_b = tree_b.remove(root_node_id_a, DropChildren);

    assert!(root_node_b.is_err());
    assert_eq!(root_node_b.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn move_node_to_root_with_old_node_id() {
    let mut tree = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_copy = root_id.clone(); //save it for later

    let _ = tree.remove(root_id, DropChildren);

    // moving node that has already been removed.
    let result = tree.move_node(&root_id_copy, ToRoot);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn move_node_to_root_in_wrong_tree() {
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
fn move_node_to_parent_with_old_parent_id() {
    let mut tree = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let child_id = tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    let root_id_copy = root_id.clone();

    let _ = tree.remove(root_id, DropChildren);

    // move node to parent that no longer exists.
    let result = tree.move_node(&child_id, ToParent(&root_id_copy));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn move_node_to_parent_with_old_child_id() {
    let mut tree = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let child_id = tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    let child_id_copy = child_id.clone();

    let _ = tree.remove(child_id, DropChildren);

    // move node to parent that no longer exists.
    let result = tree.move_node(&child_id_copy, ToParent(&root_id));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn move_node_to_parent_with_old_parent_id_and_old_child_id() {
    let mut tree = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let child_id = tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    let root_id_copy = root_id.clone();
    let child_id_copy = child_id.clone();

    let _ = tree.remove(child_id, DropChildren);
    let _ = tree.remove(root_id, DropChildren);

    // move non-existent node to parent that no longer exists.
    let result = tree.move_node(&child_id_copy, ToParent(&root_id_copy));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn move_node_to_parent_into_wrong_tree() {
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
fn move_node_to_parent_from_wrong_tree() {
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
fn move_node_to_parent_from_wrong_tree_and_into_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b = VecTreeBuilder::new().build();
    let mut tree_c: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id_a = tree_a.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_b = tree_b.insert(VecNode::new(1), AsRoot).unwrap();

    // moving child from wrong tree to parent in correct tree.
    let result = tree_c.move_node(&root_id_b, ToParent(&root_id_a));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn sort_by_with_old_id() {
    let mut tree = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_copy = root_id.clone(); //save it for later

    let _ = tree.remove(root_id, DropChildren);

    // sort children of a node that has been removed.
    let result = tree.sort_children_by(&root_id_copy, |a, b| a.data().cmp(b.data()));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn sort_by_with_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b = VecTreeBuilder::new().build();

    let _ = tree_a.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_b = tree_b.insert(VecNode::new(1), AsRoot).unwrap();

    // sort children of node from the wrong tree.
    let result = tree_a.sort_children_by(&root_id_b, |a, b| a.data().cmp(b.data()));

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn sort_by_data_with_old_id() {
    let mut tree = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_copy = root_id.clone(); //save it for later

    let _ = tree.remove(root_id, DropChildren);

    // sort children of a node that has been removed.
    let result = tree.sort_children_by_data(&root_id_copy);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn sort_by_data_with_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b = VecTreeBuilder::new().build();

    let _ = tree_a.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_b = tree_b.insert(VecNode::new(1), AsRoot).unwrap();

    // sort children of node from the wrong tree.
    let result = tree_a.sort_children_by_data(&root_id_b);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn sort_by_key_with_old_id() {
    let mut tree = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_copy = root_id.clone(); //save it for later

    let _ = tree.remove(root_id, DropChildren);

    // sort children of a node that has been removed.
    let result = tree.sort_children_by_key(&root_id_copy, |x| *x.data());

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn sort_by_key_with_wrong_tree() {
    let mut tree_a = VecTreeBuilder::new().build();
    let mut tree_b = VecTreeBuilder::new().build();

    let _ = tree_a.insert(VecNode::new(1), AsRoot).unwrap();
    let root_id_b = tree_b.insert(VecNode::new(1), AsRoot).unwrap();

    // sort children of node from the wrong tree.
    let result = tree_a.sort_children_by_key(&root_id_b, |x| *x.data());

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn swap_nodes_old_first_id() {
    let mut tree: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();

    let child_a = tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    let child_b = tree.insert(VecNode::new(3), UnderNode(&root_id)).unwrap();

    let child_a_copy = child_a.clone();  //save it for later
    tree.remove(child_a, DropChildren).unwrap();

    let result = tree.swap_nodes(&child_a_copy, &child_b, TakeChildren);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn swap_nodes_old_second_id() {
    let mut tree: VecTree<i32> = VecTreeBuilder::new().build();

    let root_id = tree.insert(VecNode::new(1), AsRoot).unwrap();

    let child_a = tree.insert(VecNode::new(2), UnderNode(&root_id)).unwrap();
    let child_b = tree.insert(VecNode::new(3), UnderNode(&root_id)).unwrap();

    let child_a_copy = child_a.clone();  //save it for later
    tree.remove(child_a, DropChildren).unwrap();

    let result = tree.swap_nodes(&child_b, &child_a_copy, TakeChildren);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn swap_sub_trees_of_different_trees() {
    let mut tree_a: VecTree<i32> = VecTreeBuilder::new().build();
    let mut tree_b: VecTree<i32> = VecTreeBuilder::new().build();

    let root_node_a = VecNode::new(1);
    let root_node_id_a = tree_a.insert(root_node_a, AsRoot).unwrap();

    let root_node_b = VecNode::new(1);
    let root_node_id_b = tree_b.insert(root_node_b, AsRoot).unwrap();

    // note use of invalid child
    let result = tree_a.swap_nodes(&root_node_id_b, &root_node_id_a, TakeChildren);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn ancestors_different_trees() {
    let mut a = VecTree::new();
    let b = VecTree::<i32>::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();

    // note usage of `b` instead of `a`
    let ancestors = b.ancestors(&root_id);

    assert!(ancestors.is_err());
    assert_eq!(ancestors.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn ancestors_old_id() {
    let mut a = VecTree::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    a.remove(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let ancestors = a.ancestors(&root_id_clone);

    assert!(ancestors.is_err());
    assert_eq!(ancestors.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn ancestor_ids_different_trees() {
    let mut a = VecTree::new();
    let b = VecTree::<i32>::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();

    // note usage of `b` instead of `a`
    let ancestors = b.ancestor_ids(&root_id);

    assert!(ancestors.is_err());
    assert_eq!(ancestors.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn ancestor_ids_old_id() {
    let mut a = VecTree::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    a.remove(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let ancestors = a.ancestor_ids(&root_id_clone);

    assert!(ancestors.is_err());
    assert_eq!(ancestors.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn children_different_trees() {
    let mut a = VecTree::new();
    let b = VecTree::<i32>::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();

    // note usage of `b` instead of `a`
    let ancestors = b.children(&root_id);

    assert!(ancestors.is_err());
    assert_eq!(ancestors.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn children_old_id() {
    let mut a = VecTree::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    a.remove(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let ancestors = a.children(&root_id_clone);

    assert!(ancestors.is_err());
    assert_eq!(ancestors.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn children_ids_different_trees() {
    let mut a = VecTree::new();
    let b = VecTree::<i32>::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();

    // note usage of `b` instead of `a`
    let ancestors = b.children_ids(&root_id);

    assert!(ancestors.is_err());
    assert_eq!(ancestors.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn children_ids_old_id() {
    let mut a = VecTree::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    a.remove(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let ancestors = a.children_ids(&root_id_clone);

    assert!(ancestors.is_err());
    assert_eq!(ancestors.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn traverse_pre_order_different_trees() {
    let mut a = VecTree::new();
    let b = VecTree::<i32>::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();

    // note usage of `b` instead of `a`
    let iter = b.traverse_pre_order(&root_id);

    assert!(iter.is_err());
    assert_eq!(iter.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn traverse_pre_order_old_id() {
    let mut a = VecTree::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    a.remove(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let iter = a.traverse_pre_order(&root_id_clone);

    assert!(iter.is_err());
    assert_eq!(iter.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn traverse_post_order_different_trees() {
    let mut a = VecTree::new();
    let b = VecTree::<i32>::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();

    // note usage of `b` instead of `a`
    let iter = b.traverse_post_order(&root_id);

    assert!(iter.is_err());
    assert_eq!(iter.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn traverse_post_order_old_id() {
    let mut a = VecTree::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    a.remove(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let iter = a.traverse_post_order(&root_id_clone);

    assert!(iter.is_err());
    assert_eq!(iter.err().unwrap(), NodeIdNoLongerValid);
}

#[test]
fn traverse_level_order_different_trees() {
    let mut a = VecTree::new();
    let b = VecTree::<i32>::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();

    // note usage of `b` instead of `a`
    let iter = b.traverse_level_order(&root_id);

    assert!(iter.is_err());
    assert_eq!(iter.err().unwrap(), InvalidNodeIdForTree);
}

#[test]
fn traverse_level_order_old_id() {
    let mut a = VecTree::new();

    let root_id = a.insert(VecNode::new(1), AsRoot).unwrap();
    // `.clone()` required to get this error
    let root_id_clone = root_id.clone();
    a.remove(root_id, DropChildren).unwrap();

    // note usage of cloned `NodeId`
    let iter = a.traverse_level_order(&root_id_clone);

    assert!(iter.is_err());
    assert_eq!(iter.err().unwrap(), NodeIdNoLongerValid);
}
