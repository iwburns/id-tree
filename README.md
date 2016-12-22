#id_tree

[![Docs.rs](https://docs.rs/id_tree/badge.svg)](https://docs.rs/id_tree)  [![Crates.io](https://img.shields.io/crates/v/id_tree.svg)](https://crates.io/crates/id_tree)  [![Build Status](https://travis-ci.org/iwburns/id-tree.svg?branch=master)](https://travis-ci.org/iwburns/id-tree)

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

let root: NodeId = tree.set_root(Node::new(0));

let child_1: NodeId = tree.insert_with_parent(Node::new(1), &root).ok().unwrap();
let child_2: NodeId = tree.insert_with_parent(Node::new(2), &root).ok().unwrap();
let child_3: NodeId = tree.insert_with_parent(Node::new(3), &child_1).ok().unwrap();

{
    let root: &Node<i32> = tree.get(&root).unwrap();
    assert_eq!(root.data(), &0);

    let children: &Vec<NodeId> = root.children();
    assert_eq!(children[0], child_1);
    assert_eq!(children[1], child_2);
}
{
    let mut child: &mut Node<i32> = tree.get_mut(&child_3).unwrap();
    assert_eq!(child.data(), &3);
    *child.data_mut() = 10;
    assert_eq!(child.data(), &10);
}

tree.move_node_to_parent(&child_3, &child_2).ok().unwrap();
assert!(tree.get(&child_2).unwrap().children().contains(&child_3));
```

## Project Goals
* Allow caller control of as many allocations as possible (through pre-allocation)
* Fast `Node` insertion and removal
* Arbitrary _Tree_ structure creation and manipulation

## Non-Goals
* Arbitrary _Graph_ structure creation and manipulation
* Comparison-based node insertion of any kind

## Contributors
* [Drakulix](https://github.com/Drakulix)
