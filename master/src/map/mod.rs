use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use common_infrastructure::Position;
use common_infrastructure::devices::{Switch, Train};
use crate::map::references::*;
use crate::map::states::{InitializedState, MapState, MapStateInitialized, MapStateUninitialized};

pub mod states;
pub mod references;



pub struct Map<T: MapState>{
    state: PhantomData<T>,
    nodes: HashMap<Position, Node<T>>,
    trains: HashMap<Train, TrainController<T>>,
    switches: HashMap<Switch, SwitchController<T>>,
}

impl<T: MapState> Map<T> {
    pub fn get_node(&self, position: Position) -> &Node<T>{
        self.nodes.get(&position).unwrap()
    }
    pub fn get_train(&self, train: Train) -> &TrainController<T>{
        self.trains.get(&train).unwrap()
    }
    pub fn get_switch(&self, switch: Switch) -> &SwitchController<T>{
        self.switches.get(&switch).unwrap()
    }
}

pub struct Node<T: MapState>{
    state: PhantomData<T>,
    position: Position,
    adjacent_nodes: AdjacentNodes<T>,
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

enum SwitchControllerOption<T: MapState>{
    NoSwitch,
    SwitchToSetStraight(T::SwitchRefType),
    SwitchToSetDiverted(T::SwitchRefType),
}

impl SwitchControllerOption<MapStateInitialized<'_>> {
    pub fn set(&self){
        match self {
            SwitchControllerOption::NoSwitch => {},
            SwitchControllerOption::SwitchToSetStraight(switch) => {
                //switch.set_straight();
            },
            SwitchControllerOption::SwitchToSetDiverted(switch) => {
                //switch.set_diverted();
            },
        }
    }
}

pub struct TrainController<T: MapState>{
    state: PhantomData<T>,
    train: Train,
}

pub struct SwitchController<T: MapState>{
    state: PhantomData<T>,
    switch: Switch,
}