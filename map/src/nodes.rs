use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use common_infrastructure::Position;
use crate::references::*;
use crate::states::{MapState, MapStateInitialized, MapStateUninitialized};
use crate::devices::SwitchControllerOption;
use anyhow::Result;
use crate::map_creation_error::MapCreationError;

#[derive(Debug,Serialize,Deserialize,Clone,PartialEq,Eq)]
pub enum NodeStatus<T: MapState>{
    /// there is no train in this node, and no train is planning to pass through it
    Unlocked,
    /// there is no train in this node, but a train is planning to pass through it
    LockedByTrain(T::TrainRefType),
    /// there is a train in this node
    OccupiedByTrain(T::TrainRefType),
}

#[derive(Debug,Serialize, Deserialize,Clone, PartialEq, Eq)]
pub struct Node<T: MapState>{
    pub position: Position,
    pub adjacent_nodes: RefCell<AdjacentNodes<T>>,
    pub status: RefCell<NodeStatus<T>>,
}

/// On our model a node can have at most 3 adjacent nodes...
/// so instead of using a vector, we use an enum to represent
/// to avoid accessing the heap for small vectors.

#[derive(Debug,Serialize,Deserialize,Clone, PartialEq, Eq)]
pub enum AdjacentNodes<T: MapState>{
    None,
    One([Link<T>;1]),
    Two([Link<T>;2]),
    Tree([Link<T>;3]),
}

impl<T: MapState> AdjacentNodes<T>{
    pub fn get_adjacent_nodes(& self) ->&[Link<T>]{
        match self {
            AdjacentNodes::None => &[],
            AdjacentNodes::One(nodes) => nodes,
            AdjacentNodes::Two(nodes) => nodes,
            AdjacentNodes::Tree(nodes) => nodes,
        }
    }
}

impl AdjacentNodes<MapStateInitialized> {
    pub fn get_link_to(&self, to: Position) -> Option<&Link<MapStateInitialized>>{
        for link in self.get_adjacent_nodes(){
            if link.node.position == to{
                return Some(link);
            }
        }
        None
    }
}

/// This struct represent a link between 2 nodes.
/// in the real world this is a rail track between 2 stations (aka tag rfid)
#[derive(Debug,Serialize,Deserialize,Clone, PartialEq, Eq)]
pub struct Link<T: MapState>{
    pub length: u32,
    pub max_speed: i8,
    pub node: T::NodeRefType,
    /// If this track travels through a switch, this is the switch
    /// it can be used to set the correct pat
    pub controller: SwitchControllerOption<T>,
    pub direction: Direction,
}

impl Link<MapStateUninitialized>{
    pub fn new(length: u32, max_speed: i8, node: UnIntiNodeRef,
               controller: SwitchControllerOption<MapStateUninitialized>, direction: Direction) -> Self {

        if max_speed < 0{
            panic!("max_speed must be positive");
        }

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
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize,Deserialize, Hash)]
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

    pub fn set_train(& self, train: UnIntiTrainRef) -> Result<()>
    {
        if *self.status.borrow() != NodeStatus::Unlocked{
            return Err(MapCreationError::new("Impossible to set train on a node that is not unlocked").into());
        }
        *self.status.borrow_mut() = NodeStatus::OccupiedByTrain(train);
        Ok(())
    }

    /// The nodes can be of two types:
    ///  - switch nodes
    ///  - non-switch nodes
    /// when a link is added it must follow a few rules:
    ///  - a non-switch node can have at most one link forward
    ///  - a non-switch node can have at most one link backward
    ///  - a switch node must have exactly two links forward, (one for each switch position)
    ///  - a switch node can have at most one link backward
    /// this function enforce all those rules, except the third one, which is enforced
    /// inside map_creation_view
    pub fn add_link(&self, to: UnIntiNodeRef, direction: Direction, max_speed: i8,
                    length: u32, controller: SwitchControllerOption<MapStateUninitialized>) -> Result<()>{

        let mut node_backward = None;
        let mut node_forward = None;

        let adjacent_nodes = self.adjacent_nodes.borrow();

        for node in adjacent_nodes.get_adjacent_nodes(){
            if node.node.position == to.position{
                return Err(MapCreationError::new("Impossible to add a link to a node that is already linked").into());
            }
            match node.direction{
                Direction::Forward => node_forward = Some(node),
                Direction::Backward => node_backward = Some(node),
            }
        }

        if direction == Direction::Backward && node_backward.is_some(){
            return Err(MapCreationError::new("Impossible to add a link backward to a node that already has a link backward").into());
        }

        // be shure that if two links are added to the same node, forward, they points to the same switch,
        // and one of them is diverted and the other is straight
        if direction == Direction::Forward && node_forward.is_some(){
            let node_forward = node_forward.unwrap();
            match (&node_forward.controller,&controller) {
                (SwitchControllerOption::SwitchToSetDiverted(s1), SwitchControllerOption::SwitchToSetStraight(s2)) => {
                    if s1 != s2{
                        return Err(MapCreationError::new("the node dose not respect the switch pattern").into());
                    }
                }
                (SwitchControllerOption::SwitchToSetStraight(s1), SwitchControllerOption::SwitchToSetDiverted(s2)) => {
                    if s1 != s2{
                        return Err(MapCreationError::new("the node dose not respect the switch pattern").into());
                    }
                }
                (_,_) => return Err(MapCreationError::new("the node dose not respect the switch pattern").into())

            }
        }

        // drop this, otherwise the next line will panic
        // because we are borrowing adjacent_nodes as immutable
        drop(adjacent_nodes);

        self.adjacent_nodes.borrow_mut().add_link(to, controller, direction, max_speed, length)?;

        Ok(())
    }

    pub fn get_next_available_direction(&self) -> Result<Direction>{
        let mut has_forward = false;
        let mut has_backward = false;
        for link in self.adjacent_nodes.borrow().get_adjacent_nodes(){
            if link.direction == Direction::Forward{
                has_forward = true;
            }
            if link.direction == Direction::Backward{
                has_backward = true;
            }
        }
        if !has_forward{
            return Ok(Direction::Forward);
        }
        if !has_backward{
            return Ok(Direction::Backward);
        }
        Err(MapCreationError::new("The node already has two links").into())
    }
}


impl AdjacentNodes<MapStateUninitialized> {
    pub fn add_link(&mut self, to: UnIntiNodeRef, controller: SwitchControllerOption<MapStateUninitialized>,
                    direction: Direction, max_speed: i8, length: u32) -> Result<()>{

        for node in self.get_adjacent_nodes(){
            if node.node.position == to.position{
                return Err(MapCreationError::new("Impossible to add a link to a node that is already linked").into());
            }
        }

        let new_link = Link::new(length, max_speed, to, controller, direction);

        *self = match self.clone() {
            AdjacentNodes::None => AdjacentNodes::One([new_link]),
            AdjacentNodes::One([l1]) => AdjacentNodes::Two([l1, new_link]),
            AdjacentNodes::Two([l1,l2]) => AdjacentNodes::Tree([l1,l2, new_link]),
            AdjacentNodes::Tree(_) => return Err(MapCreationError::new("Impossible to add more than 3 links to a node").into())
        };

        Ok(())

    }
}