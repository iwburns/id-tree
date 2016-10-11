extern crate id_tree;

use id_tree::NodeId;
use id_tree::Node;
use id_tree::TreeBuilder;
use id_tree::Tree;

fn main() {
    let mut tree: Tree<i32> = TreeBuilder::new()
        .with_node_capacity(3)
        .build();

    let root_node = Node::new(1);
    let child_1 = Node::new(2);
    let child_3 = Node::new(3);
    let grandchild_4 = Node::new(4);
    let grandchild_5 = Node::new(5);

    let root_id: NodeId = tree.set_root(root_node);
    let child_1_id: NodeId = tree.insert_with_parent(child_1, &root_id).unwrap();
    tree.insert_with_parent(child_3, &root_id).unwrap();
    tree.insert_with_parent(grandchild_4, &child_1_id).unwrap();
    tree.insert_with_parent(grandchild_5, &child_1_id).unwrap();

    println!("Post-order:");
    print_node_post_order(&tree, &root_id);
    println!("Pre-order:");
    print_node_pre_order(&tree, &root_id);
}

fn print_node_post_order(tree: &Tree<i32>, node_id: &NodeId) {
    let node_ref: &Node<i32> = tree.get(node_id).unwrap();

    for child_id in node_ref.children() {
        print_node_post_order(tree, &child_id);
    }

    println!("{}", node_ref.data());
}

fn print_node_pre_order(tree: &Tree<i32>, node_id: &NodeId) {
    let node_ref: &Node<i32> = tree.get(node_id).unwrap();

    println!("{}", node_ref.data());

    for child_id in node_ref.children() {
        print_node_pre_order(tree, &child_id);
    }
}
