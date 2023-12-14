use std::marker::PhantomData;
use common_infrastructure::Position;
use crate::map::references::*;
use crate::map::states::{MapState, MapStateInitialized};
use crate::map::devices::SwitchControllerOption;

pub enum NodeStatus<T: MapState>{
    /// there is no train in this node, and no train is planning to pass through it
    Unlocked,
    /// there is no train in this node, but a train is planning to pass through it
    LockedByTrain(T::TrainRefType),
    /// there is a train in this node
    OccupiedByTrain(T::TrainRefType),
}

pub struct Node<T: MapState>{
    state: PhantomData<T>,
    pub position: Position,
    adjacent_nodes: AdjacentNodes<T>,
    status: NodeStatus<T>,
}

/// On our model a node can have at most 3 adjacent nodes...
/// so instead of using a vector, we use an enum to represent
/// to avoid accessing the heap for small vectors.
enum AdjacentNodes<T: MapState>{
    None,
    One([T::NodeRefType;1]),
    Two([T::NodeRefType;2]),
    Tree([T::NodeRefType;3]),
}

impl<'a> AdjacentNodes<MapStateInitialized<'a>>{
    fn get_adjacent_nodes(&'a self) ->&[IntiNodeRef<'a>]{
        match self {
            AdjacentNodes::None => &[],
            AdjacentNodes::One(nodes) => nodes,
            AdjacentNodes::Two(nodes) => nodes,
            AdjacentNodes::Tree(nodes) => nodes,
        }
    }
}

/// This struct represent a link between 2 nodes.
/// in the real world this is a rail track between 2 stations (aka tag rfid)
pub struct Link<T: MapState>{
    state: PhantomData<T>,
    length: u32,
    max_speed: u32,
    node: T::NodeRefType,
    /// If this track travels through a switch, this is the switch
    /// it can be used to set the correct pat
    controller: SwitchControllerOption<T>,
    direction: Direction,
}

/// When you are in a node (aka station) you can go only int two directions
/// this enum represent those 2 directions, forward and backward.
enum Direction{
    Forward,
    Backward,
}
