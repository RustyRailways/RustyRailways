use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use common_infrastructure::Position;
use common_infrastructure::devices::{Switch, Train};
use crate::map::references::*;
use crate::map::states::{InitializedState, MapState, MapStateInitialized, MapStateUninitialized};
use devices::{SwitchController, TrainController};
use nodes::Node;

pub mod states;
pub mod references;

pub mod nodes;

/// Contains Switch and Trains
pub mod devices;


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

