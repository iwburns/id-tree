mod vec_tree;

pub use self::vec_tree::*;

use std::cmp::Ordering;
use behaviors::*;
use iterators::*;
use NodeId;
use NodeIdError;

pub trait Tree<T> {
    type NodeType;

    fn new() -> Self;

    fn insert(
        &mut self,
        node: Self::NodeType,
        behavior: InsertBehavior,
    ) -> Result<NodeId, NodeIdError>;

    fn get(&self, node_id: &NodeId) -> Result<&Self::NodeType, NodeIdError>;

    fn get_mut(&mut self, node_id: &NodeId) -> Result<&Self::NodeType, NodeIdError>;

    fn remove(
        &mut self,
        node_id: NodeId,
        behavior: RemoveBehavior,
    ) -> Result<Self::NodeType, NodeIdError>;

    fn move_node(&mut self, node_id: &NodeId, behavior: MoveBehavior) -> Result<(), NodeIdError>;

    fn sort_children_by<F>(&mut self, node_id: &NodeId, compare: F) -> Result<(), NodeIdError>
    where
        F: FnMut(&Self::NodeType, &Self::NodeType) -> Ordering;

    fn sort_children_by_data(&mut self, node_id: &NodeId) -> Result<(), NodeIdError>
    where
        T: Ord;

    fn sort_children_by_key<K, F>(&mut self, node_id: &NodeId, f: F) -> Result<(), NodeIdError>
    where
        K: Ord,
        F: FnMut(&Self::NodeType) -> K;

    fn swap_nodes(
        &mut self,
        first_id: &NodeId,
        second_id: &NodeId,
        behavior: SwapBehavior,
    ) -> Result<(), NodeIdError>;

    fn root_node_id(&self) -> Option<&NodeId>;

    fn ancestors(&self, node_id: &NodeId) -> Result<Ancestors<T>, NodeIdError>;

    fn ancestor_ids(&self, node_id: &NodeId) -> Result<AncestorIds<T>, NodeIdError>;

    fn children(&self, node_id: &NodeId) -> Result<Children<T>, NodeIdError>;

    fn children_ids(&self, node_id: &NodeId) -> Result<ChildrenIds, NodeIdError>;

    fn traverse_pre_order(&self, node_id: &NodeId) -> Result<PreOrderTraversal<T>, NodeIdError>;

    fn traverse_post_order(&self, node_id: &NodeId) -> Result<PostOrderTraversal<T>, NodeIdError>;

    fn traverse_level_order(&self, node_id: &NodeId)
        -> Result<LevelOrderTraversal<T>, NodeIdError>;
}
