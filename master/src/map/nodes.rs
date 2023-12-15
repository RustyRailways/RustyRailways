use std::cell::RefCell;
use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use common_infrastructure::devices::Switch;
use common_infrastructure::Position;
use crate::map::references::*;
use crate::map::states::{MapState, MapStateInitialized, MapStateUninitialized};
use crate::map::devices::SwitchControllerOption;
use crate::map::Map;

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Eq)]
pub enum NodeStatus<T: MapState>{
    /// there is no train in this node, and no train is planning to pass through it
    Unlocked,
    /// there is no train in this node, but a train is planning to pass through it
    LockedByTrain(T::TrainRefType),
    /// there is a train in this node
    OccupiedByTrain(T::TrainRefType),
}

#[derive(Debug, Serialize,Deserialize,Clone)]
pub struct Node<T: MapState>{
    pub position: Position,
    pub adjacent_nodes: RefCell<AdjacentNodes<T>>,
    pub status: RefCell<NodeStatus<T>>,
}

/// On our model a node can have at most 3 adjacent nodes...
/// so instead of using a vector, we use an enum to represent
/// to avoid accessing the heap for small vectors.

#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum AdjacentNodes<T: MapState>{
    None,
    One([Link<T>;1]),
    Two([Link<T>;2]),
    Tree([Link<T>;3]),
}

impl<T: MapState> AdjacentNodes<T>{
    fn get_adjacent_nodes(& self) ->&[Link<T>]{
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
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Link<T: MapState>{
    pub length: u32,
    pub max_speed: u32,
    pub node: T::NodeRefType,
    /// If this track travels through a switch, this is the switch
    /// it can be used to set the correct pat
    pub controller: SwitchControllerOption<T>,
    pub direction: Direction,
}

impl Link<MapStateUninitialized>{
    pub fn new(length: u32, max_speed: u32, node: UnIntiNodeRef,
               controller: SwitchControllerOption<MapStateUninitialized>, direction: Direction) -> Self {

        Link {
            length,
            max_speed,
            node,
            controller,
            direction,
        }
    }
}


/// When you are in a node (aka station) you can go only int two directions
/// this enum represent those 2 directions, forward and backward.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize,Deserialize)]
pub enum Direction{
    Forward,
    Backward,
}


////////////////////// Implementation of uninitialized Node //////////////////////

impl Node<MapStateUninitialized>{
    pub fn new(position: Position) -> Self{
        Node{
            position,
            adjacent_nodes: AdjacentNodes::None.into(),
            status: NodeStatus::Unlocked.into(),
        }
    }

    pub fn set_train(& self, train: UnIntiTrainRef) -> Result<(),&str>
    {
        if *self.status.borrow() != NodeStatus::Unlocked{
            return Err("Impossible to set train on a node that is not unlocked");
        }
        *self.status.borrow_mut() = NodeStatus::OccupiedByTrain(train);
        Ok(())
    }

    pub fn add_link(&self, to: UnIntiNodeRef, direction: Direction, max_speed: u32, length: u32, controller: SwitchControllerOption<MapStateUninitialized>) -> Result<(),&str>{

        self.adjacent_nodes.borrow_mut().add_link(to, controller, direction, max_speed, length)?;

        Ok(())
    }
}


////////////////////// Implementation of initialized Node //////////////////////

impl Node<MapStateInitialized>{

    fn new(position: Position) -> Self{
        Node{
            position,
            adjacent_nodes: AdjacentNodes::None.into(),
            status: NodeStatus::Unlocked.into(),
        }
    }
}

impl AdjacentNodes<MapStateUninitialized> {
    fn add_link(&mut self, to: UnIntiNodeRef, controller: SwitchControllerOption<MapStateUninitialized>, direction: Direction, max_speed: u32, length: u32) -> Result<(),&'static str>{

        for node in self.get_adjacent_nodes(){
            if node.node.position == to.position{
                return Err("Impossible to add a link to a node that is already linked");
            }
        }

        let new_link = Link::new(length, max_speed, to, controller, direction);

        *self = match self.clone() {
            AdjacentNodes::None => AdjacentNodes::One([new_link]),
            AdjacentNodes::One([l1]) => AdjacentNodes::Two([l1, new_link]),
            AdjacentNodes::Two([l1,l2]) => AdjacentNodes::Tree([l1,l2, new_link]),
            AdjacentNodes::Tree(_) => return Err("Impossible to add more than 3 links to a node")
        };

        Ok(())

    }
}