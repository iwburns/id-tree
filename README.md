#id_tree

[![Docs.rs](https://docs.rs/id_tree/badge.svg)](https://docs.rs/id_tree)  [![Crates.io](https://img.shields.io/crates/v/id_tree.svg)](https://crates.io/crates/id_tree)

A library for creating and modifying Tree structures.

# Overview
In this implementation, the `Tree` owns all of the `Node`s and all inter-`Node` relationships are
managed with `NodeId`s.

`Tree`s in this library are "just" trees.  They do not allow cycles.  They do not allow
the creation of arbitrary Graph structures.  There is no weight associated with edges between
`Node`s.  In addition, each `Node` can have an arbitrary number of child `Node`s.

It is important to note that this library does not support comparison-based `Node` insertion.
In other words, this is not a Binary Search Tree (or any other kind of search tree) library.
It is purely a library for storing data in a hierarchical manner.  The caller must know the
structure that they wish to build and then use this library to do so;  this library will not
make those structural decisions for you.

## Example Usage
```
use id_tree::*;

let mut tree: Tree<i32> = Tree::new();

let root_id: NodeId = tree.set_root(Node::new(1));
let child_id: NodeId = tree.insert_with_parent(Node::new(2), &root_id).ok().unwrap();

{
    let root: &Node<i32> = tree.get(&root_id).unwrap();
    assert_eq!(root.data(), &1);
    
    let first_child: &NodeId = root.children().get(0).unwrap();
    assert_eq!(first_child, &child_id);
}
{
    let mut child: &mut Node<i32> = tree.get_mut(&child_id).unwrap();
    assert_eq!(child.data_mut(), &mut 2);
}
```

## Project Goals
* Allow caller control of as many allocations as possible (through pre-allocation)
* Fast `Node` insertion and removal
* Arbitrary _Tree_ structure creation and manipulation

## Non-Goals
* Arbitrary _Graph_ structure creation and manipulation
* Comparison-based node insertion of any kind
