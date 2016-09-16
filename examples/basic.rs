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

    let root_id: NodeId = tree.set_root(root_node);
    let child_1_id: NodeId = tree.add_child(root_id, child_1);
    let child_2_id: NodeId = tree.add_child(root_id, child_2);

    println!("{}", tree.get(root_id).unwrap().data());
    println!("{}", tree.get(child_1_id).unwrap().data());
    println!("{}", tree.get(child_2_id).unwrap().data());
}
