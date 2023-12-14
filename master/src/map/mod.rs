use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use common_infrastructure::Position;
use common_infrastructure::devices::{Switch, Train};
use crate::map::references::*;
use crate::map::states::{ReferenceStateInitialized, MapState, MapStateInitialized, MapStateUninitialized};
use devices::{SwitchController, TrainController};
use nodes::Node;

pub mod states;
pub mod references;

pub mod nodes;

/// Contains Switch and Trains
pub mod devices;

pub mod initialization;

pub struct Map<'a,T: MapState<'a>>{
    state: PhantomData<T>,
    nodes: HashMap<Position, Node<'a,T>>,
    trains: HashMap<Train, TrainController<'a,T>>,
    switches: HashMap<Switch, SwitchController<'a,T>>,
}

impl<'a, T: MapState<'a>> Map<'a, T> {
    pub fn get_node(&self, position: Position) -> &Node<'a, T>{
        self.nodes.get(&position).unwrap()
    }
    pub fn get_train(&self, train: Train) -> &TrainController<'a, T>{
        self.trains.get(&train).unwrap()
    }
    pub fn get_switch(&self, switch: Switch) -> &SwitchController<'a, T>{
        self.switches.get(&switch).unwrap()
    }
}

impl<'a> Map<'a,MapStateUninitialized>{

    pub fn new() -> Self{
        Map{
            state: PhantomData,
            nodes: HashMap::new(),
            trains: HashMap::new(),
            switches: HashMap::new(),
        }
    }

    pub fn initialize(self) -> Map<'a,MapStateInitialized>{
        todo!()
    }


}