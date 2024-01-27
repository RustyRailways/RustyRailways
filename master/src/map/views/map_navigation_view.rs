use std::cell::Ref;
use std::ops::Deref;
use common_infrastructure::devices::Train;
use common_infrastructure::Position;
use crate::map::Map;
use crate::map::nodes::{AdjacentNodes, Link, Node, NodeStatus};
use crate::map::states::MapStateInitialized;
use anyhow::Result;
/// # Map Navigation View
/// struct that allows to navigate the map's graph easily.
/// ## Notes
/// the others views of map uses some "Rest API"
/// this means that they are stateless, when one function is called,
/// it borrow the map, do some stuff, and "un-borrow" the map.
/// this is not the case for this view, because, when you het a node
/// you are able to iterate over it's neighbors, this means that
/// the map is still borrowed, so you can't call another function.
/// This implies that you can't use "mutable" functions, while
/// this view is constructed, otherwise the program will panic.
/// The functions that can cause this are:
///  - set_train_speed
///  - move_train
pub struct MapNavigationView<'a>{
    map: Ref<'a, Map<MapStateInitialized>>
}

impl<'a> MapNavigationView<'a>{
    pub fn new(map: Ref<'a, Map<MapStateInitialized>>) -> Self{
        MapNavigationView{ map }
    }

    pub fn get_node(&self, position: Position) -> Result<NavigationNode>{
        let node = self.map.get_node(position)?;
        Ok(NavigationNode::new(node))
    }
}

/// an easy to use interface to represent a node when navigating the map
pub struct NavigationNode<'a> {
    node: &'a Node<MapStateInitialized>,
    adjacent_nodes: Ref<'a,AdjacentNodes<MapStateInitialized>>
}

impl<'a> NavigationNode<'a>{
    fn new(node: &'a Node<MapStateInitialized>) -> Self{
        Self{
            node,
            adjacent_nodes: node.adjacent_nodes.borrow()
        }
    }

    /// # Returns the position of the node
    pub fn get_position(&self) -> Position{
        self.node.position
    }

    /// # Returns true if the node is occupied by a train
    pub fn is_occupied(&self) -> bool{
        match self.node.status.borrow().deref() {
            NodeStatus::OccupiedByTrain(_) => true,
            _ => false
        }
    }

    /// # Returns true if the node is locked by a train
    pub fn is_locked(&self) -> bool{
        match self.node.status.borrow().deref() {
            NodeStatus::LockedByTrain(_) => true,
            _ => false
        }
    }

    /// # Returns true if the node is unlocked
    pub fn is_unlocked(&self) -> bool{
        match self.node.status.borrow().deref() {
            NodeStatus::Unlocked => true,
            _ => false
        }
    }

    /// # Returns the train that is locking the node
    /// if the node is not locked, returns None
    pub fn get_locking_train(&self) -> Option<Train>{
        match self.node.status.borrow().deref() {
            NodeStatus::LockedByTrain(train_id) => Some(train_id.deref().train),
            _ => None
        }
    }

    /// # Returns the train that is occupying the node
    /// if the node is not occupied, returns None
    pub fn get_occupying_train(&self) -> Option<Train>{
        match self.node.status.borrow().deref() {
            NodeStatus::OccupiedByTrain(train_id) => Some(train_id.deref().train),
            _ => None
        }
    }

    /// # Returns true if the train can pass through the node
    pub fn can_train_pass(&self, train: Train) -> bool{
        match self.node.status.borrow().deref() {
            NodeStatus::LockedByTrain(train_id) => train_id.deref().train == train,
            NodeStatus::Unlocked => true,
            NodeStatus::OccupiedByTrain(train_id) => train_id.deref().train == train,
        }
    }

    /// # Returns an iterator over the adjacent nodes
    pub fn neighbours_iter(&'a self) -> NavigationNodeIterator<'a>{
        return NavigationNodeIterator::new(&self.adjacent_nodes);
    }
}

/// an easy to use interface to represent an adjacent node when navigating the map
pub struct NavigationNodeIterator<'a>{
    iter: std::slice::Iter<'a, Link<MapStateInitialized>>
}


impl<'a> Iterator for NavigationNodeIterator<'a>{
    type Item = AdjacentNodeLink<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|link| link.into())
    }
}


impl<'a> NavigationNodeIterator<'a> {
    fn new(adj: &'a AdjacentNodes<MapStateInitialized>) -> Self{
        Self{
            iter: adj.get_adjacent_nodes().iter()
        }
    }
}
/// an easy to use interface to represent an adjacent node when navigating the map
pub struct AdjacentNodeLink<'a>{
    link: &'a Link<MapStateInitialized>
}

impl AdjacentNodeLink<'_>{
    /// # Returns the position of the node
    pub fn get_position(&self) -> Position{
        self.link.node.position
    }

    /// # Returns the adjacent node the link is pointing to
    pub fn get_node(&self) -> NavigationNode<'_>{
        NavigationNode::new(&self.link.node)
    }

    /// # Return the length of the link
    pub fn get_length(&self) -> u32{
        self.link.length
    }
}

impl<'a> From<&'a Link<MapStateInitialized>> for AdjacentNodeLink<'a>{
    fn from(link: &'a Link<MapStateInitialized>) -> Self {
        Self{link}
    }
}