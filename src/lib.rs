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
//! ## Project Goals
//! * Allow caller control of as many allocations as possible (through pre-allocation)
//! * Fast `Node` insertion and removal
//! * Arbitrary _Tree_ structure creation and manipulation
//!
//! ## Non-Goals
//! * Arbitrary _Graph_ structure creation and manipulation
//! * Comparison-based node insertion of any kind
//!
//! #### Drawbacks of this Library
//! Sadly, Rust's ownership/reference system is sidestepped a bit by this implementation and this
//! can cause issues if the caller doesn't pay attention to what they are doing with `NodeId`s.
//!
//! Because `Tree`s pass out `NodeId`s as `Node`s are inserted, several issues can occur:
//!
//! 1. If a `Node` is removed, the `NodeId` that previously identified it now points nothing
//! (technically a `None` value in this case).
//! 2. If a `Node` is removed and then another is inserted later, the "new" `NodeId` that is
//! returned can (and will) be the same `NodeId` that was used to identify a different `Node`
//! previously.
//!
//! The above issues may seem like deal-breakers, but if this library is used properly they can be
//! avoided entirely and never cause issues.
//!
//! To mitigate the above issues, this library does the following:
//!
//! 1. All `Tree` methods that **read** or **insert** data take `&NodeId`s instead of `NodeId`s.
//! 2. All `Tree` methods that **remove** data take `NodeId`s instead of `&NodeId`s.
//! 2. `NodeId`s themselves are `Clone`, but not `Copy`.
//!
//! This means we have "almost safe references" that you can clone if you choose to.  The resulting
//! behavior is that unless the caller **explicitly `Clone`s a `NodeId`** they should never be in a
//! situation where they accidentally hold onto a `NodeId` too long.
//!
//! This _does_ transfer some of the burden to the caller, but any errors should be fairly easy to
//! sort out because an explicit `Clone` is required for such an error to occur.
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
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct NodeId {
    tree_id: ProcessUniqueId,
    index: usize,
}

trait MutableNode {
    fn set_parent(&mut self, parent: Option<NodeId>);
    fn add_child(&mut self, child: NodeId);
    fn children_mut(&mut self) -> &mut Vec<NodeId>;
}
