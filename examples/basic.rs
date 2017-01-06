extern crate id_tree;

use id_tree::NodeId;
use id_tree::Node;
use id_tree::TreeBuilder;
use id_tree::Tree;
use id_tree::InsertBehavior;

fn main() {
    let mut tree: Tree<i32> = TreeBuilder::new()
        .with_node_capacity(5)
        .build();

    let root_id: NodeId = tree.insert(Node::new(0), InsertBehavior::AsRoot).unwrap();
    let child_id: NodeId = tree.insert(Node::new(1), InsertBehavior::UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(2), InsertBehavior::UnderNode(&root_id)).unwrap();
    tree.insert(Node::new(3), InsertBehavior::UnderNode(&child_id)).unwrap();
    tree.insert(Node::new(4), InsertBehavior::UnderNode(&child_id)).unwrap();

    println!("Post-order:");
    print_nodes_post_order(&tree, &root_id);
    println!("");
    println!("Pre-order:");
    print_nodes_pre_order(&tree, &root_id);
    println!("");
}

fn print_nodes_post_order(tree: &Tree<i32>, node_id: &NodeId) {
    let node_ref: &Node<i32> = tree.get(node_id).unwrap();

    for child_id in node_ref.children() {
        print_nodes_post_order(tree, &child_id);
    }

    print!("{}, ", node_ref.data());
}

fn print_nodes_pre_order(tree: &Tree<i32>, node_id: &NodeId) {
    let node_ref: &Node<i32> = tree.get(node_id).unwrap();

    print!("{}, ", node_ref.data());

    for child_id in node_ref.children() {
        print_nodes_pre_order(tree, &child_id);
    }
}
