//! A library for creating and modifying Tree structures.
//!
//! # Overview
//! In this implementation, the `Tree` owns all of the `Node`s and all inter-`Node` relationships are
//! managed with `NodeId`s.
//!
//! `Tree`s in this library are "just" trees.  They do not allow cycles.  They do not allow
//! the creation of arbitrary Graph structures.  There is no weight associated with edges between
//! `Node`s.  In addition, each `Node` can have an arbitrary number of child `Node`s.
//!
//! It is important to note that this library does not support comparison-based `Node` insertion.
//! In other words, this is not a Binary Search Tree (or any other kind of search tree) library.
//! It is purely a library for storing data in a hierarchical manner.  The caller must know the
//! structure that they wish to build and then use this library to do so;  this library will not
//! make those structural decisions for you.
//!
//! ### Project Goals
//! * Allow caller control of as many allocations as possible (through pre-allocation)
//! * Fast `Node` insertion and removal
//!
//! ### Non-Goals
//! * Arbitrary graph creation and manipulation
//! * Comparison-based node insertion of any kind
//!
//! #### Drawbacks of this Library
//! Rust's ownership system is sidestepped a bit by this implementation.
//!
//! Because `Tree`s give out `NodeId`s to identify `Node`s when they are inserted, those `NodeId`s
//! will - by their very nature - become invalid when the `Node` they refer to is removed from the
//! `Tree`.  This is because they are simply identifiers and not real references.  In addition (and
//! causing even more issues), if another `Node` is inserted, the old `NodeId` will be reused to
//! save space meaning it now refers to a different `Node` than it did originally.
//!
//! This means that some of the burden falls on the caller to make sure that old `NodeId`s aren't
//! kept around.  This library will return `Result`s where errors are possible, but it has no way of
//! letting the caller know that they are using a `NodeId` that has been re-purposed.  Sadly this is
//! a limitation of this type of implementation itself and cannot fully be avoided.
//!
//! This really just means that the caller will have to pay a bit more attention to the `NodeId`s
//! that are maintained throughout the life of their program.
//!
//! -----------------------------------------------------------------------------------------------
//!
//! _**Disclaimer:** This library should be considered a Work-in-Progress until it reaches v1.0.0.
//! Breaking changes will be avoided at all costs, but until v1.0.0 hits, they are a definite
//! possibility. With that in mind, it would be wise to find a version number that works for you
//! (preferably whatever the current version is when you read this) and stick with it until you are
//! ready to upgrade to the next version._
//!

extern crate snowflake;
use self::snowflake::ProcessUniqueId;

mod error;
mod node;
mod tree;

pub use error::NodeIdError;
pub use node::NodeBuilder;
pub use node::Node;
pub use tree::TreeBuilder;
pub use tree::Tree;

///
/// An identifier used to differentiate between `Node`s within a `Tree`.
///
/// `NodeId`s are not something that you will ever have to worry about generating yourself.  `Tree`s
/// generate `NodeId`s as you insert `Node`s into them.
///
/// `NodeId`s are also specific to the `Tree` that generated them.  This means that if you have two `Tree`s
/// `A` and `B`, there's no worry of trying to access a `Node` in `A` with an identifier that came
/// from `B`.  Doing so will return a `NodeIdError` instead of returning the wrong `Node`.
///
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: usize,
}

trait MutableNode {
    fn set_parent(&mut self, parent: Option<NodeId>);
    fn add_child(&mut self, child: NodeId);
    fn children_mut(&mut self) -> &mut Vec<NodeId>;
}
