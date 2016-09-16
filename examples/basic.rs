extern crate id_tree;

use id_tree::NodeId;
use id_tree::Node;
use id_tree::TreeBuilder;
use id_tree::Tree;

fn main() {
    let mut tree: Tree<i32> = TreeBuilder::new()
        .with_capacity(3)
        .build();

    let root_node = Node::new(1);
    let child_1 = Node::new(2);
    let child_2 = Node::new(3);
    let grandchild_4 = Node::new(4);
    let grandchild_5 = Node::new(5);

    let root_id: NodeId = tree.set_root(root_node);
    let child_1_id: NodeId = tree.add_child(root_id, child_1);
    tree.add_child(root_id, child_2);
    tree.add_child(child_1_id, grandchild_4);
    tree.add_child(child_1_id, grandchild_5);

    print_children_post_order(&tree, root_id);
    println!("{}", tree.get(root_id).unwrap().data());
}

fn print_children_post_order(tree: &Tree<i32>, node_id: NodeId) {
    let node_ref: &Node<i32> = tree.get(node_id).unwrap();

    for child_id in node_ref.children() {
        print_children_post_order(tree, *child_id);
        println!("{}", tree.get(*child_id).unwrap().data());
    }
}
