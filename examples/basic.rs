extern crate id_tree;

use id_tree::NodeId;
use id_tree::Node;
use id_tree::TreeBuilder;
use id_tree::Tree;

fn main() {
    let mut tree: Tree<i32> = TreeBuilder::new()
        .with_node_capacity(5)
        .build();

    let root_id: NodeId = tree.set_root(Node::new(0));
    let child_1_id: NodeId = tree.insert_with_parent(Node::new(1), &root_id).unwrap();
    tree.insert_with_parent(Node::new(2), &root_id).unwrap();
    tree.insert_with_parent(Node::new(3), &child_1_id).unwrap();
    tree.insert_with_parent(Node::new(4), &child_1_id).unwrap();

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
