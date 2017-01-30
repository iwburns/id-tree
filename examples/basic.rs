extern crate id_tree;

use id_tree::*;

fn main() {
    use id_tree::InsertBehavior::*;

    //      0
    //     / \
    //    1   2
    //   / \
    //  3   4
    let mut tree: Tree<i32> = TreeBuilder::new()
        .with_node_capacity(5)
        .build();

    let root_id: NodeId = tree.insert(Node::new(0), AsRoot).unwrap();
    let child_id: NodeId = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(3), UnderNode(&child_id)).unwrap();
    tree.insert(Node::new(4), UnderNode(&child_id)).unwrap();

    println!("Post-order:");
    print_post_order(&tree, &root_id);
    println!();

    println!("Pre-order:");
    print_pre_order(&tree, &root_id);
    println!();
}

fn print_pre_order(tree: &Tree<i32>, node_id: &NodeId) {
    let node_ref = tree.get(node_id).unwrap();

    print!("{}, ", node_ref.data());

    for child_id in node_ref.children() {
        print_pre_order(tree, &child_id);
    }
}

fn print_post_order(tree: &Tree<i32>, node_id: &NodeId) {
    let node_ref = tree.get(node_id).unwrap();

    for child_id in node_ref.children() {
        print_post_order(tree, &child_id);
    }

    print!("{}, ", node_ref.data());
}
