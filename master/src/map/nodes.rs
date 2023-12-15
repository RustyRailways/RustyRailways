use std::cell::RefCell;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use common_infrastructure::Position;
use crate::map::references::*;
use crate::map::states::{MapState, MapStateInitialized, MapStateUninitialized};
use crate::map::devices::SwitchControllerOption;
use crate::map::Map;

#[derive(Debug,Serialize)]
pub enum NodeStatus<'a,T: MapState<'a>>{
    /// there is no train in this node, and no train is planning to pass through it
    Unlocked,
    /// there is no train in this node, but a train is planning to pass through it
    LockedByTrain(T::TrainRefType),
    /// there is a train in this node
    OccupiedByTrain(T::TrainRefType),
}

#[derive(Debug, Serialize)]
pub struct Node<'a, T: MapState<'a>>{
    state: PhantomData<T>,
    pub position: Position,
    adjacent_nodes: RefCell<AdjacentNodes<'a,T>>,
    status: RefCell<NodeStatus<'a,T>>,
}

/// On our model a node can have at most 3 adjacent nodes...
/// so instead of using a vector, we use an enum to represent
/// to avoid accessing the heap for small vectors.
#[derive(Debug,Serialize)]
pub enum AdjacentNodes<'a,T: MapState<'a>>{
    None,
    One([T::NodeRefType;1]),
    Two([T::NodeRefType;2]),
    Tree([T::NodeRefType;3]),
}

impl<'a> AdjacentNodes<'a,MapStateInitialized>{
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
pub struct Link<'a,T: MapState<'a>>{
    pub length: u32,
    pub max_speed: u32,
    pub node: T::NodeRefType,
    /// If this track travels through a switch, this is the switch
    /// it can be used to set the correct pat
    pub controller: SwitchControllerOption<'a,T>,
    pub direction: Direction,
}

/// When you are in a node (aka station) you can go only int two directions
/// this enum represent those 2 directions, forward and backward.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize,Deserialize)]
pub enum Direction{
    Forward,
    Backward,
}


////////////////////// Implementation of uninitialized Node //////////////////////

impl Node<'_,MapStateUninitialized>{
    pub fn new(position: Position) -> Self{
        Node{
            state: PhantomData,
            position,
            adjacent_nodes: AdjacentNodes::None.into(),
            status: NodeStatus::Unlocked.into(),
        }
    }

    pub fn set_train(&mut self, train: UnIntiTrainRef){
        self.status = NodeStatus::OccupiedByTrain(train).into();
    }
}


////////////////////// Implementation of initialized Node //////////////////////

impl<'a> Node<'a,MapStateInitialized>{

    fn new(position: Position) -> Self{
        Node{
            state: PhantomData,
            position,
            adjacent_nodes: AdjacentNodes::None.into(),
            status: NodeStatus::Unlocked.into(),
        }
    }


    pub fn complete_initialization(&'a self, node: Node<'_,MapStateInitialized>, map: &'a Map<'a,MapStateInitialized>){
        assert_eq!(self.position, node.position);
    }

}