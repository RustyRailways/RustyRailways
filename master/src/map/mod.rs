use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use serde::{Deserialize, Serialize};
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

/// Unfortunately, as of today (december 2023) there is no way to derive
/// a trait for a generic structure only for some specific trait bounds.
/// this means that our struct `Map` can't derive `Serialize` and `Deserialize`
/// as it contains references to `Node`, `TrainController` and `SwitchController`
/// in the initialized version.
/// To solve this issue we implemented the serialization and deserialization manually here.
pub mod serde_impls;

#[derive(Debug,Serialize,Deserialize)]
pub struct Map<T: MapState>{
    nodes: HashMap<Position, Node<T>>,
    trains: HashMap<Train, TrainController>,
    switches: HashMap<Switch, SwitchController>,
}

impl<T: MapState> Map<T> {
    pub fn get_node(&self, position: Position) -> &Node<T>{
        self.nodes.get(&position).unwrap()
    }
    pub fn get_train(&self, train: Train) -> &TrainController{
        self.trains.get(&train).unwrap()
    }
    pub fn get_switch(&self, switch: Switch) -> &SwitchController{
        self.switches.get(&switch).unwrap()
    }
}

impl Map<MapStateUninitialized>{
    pub fn new() -> Self{
        Map{
            nodes: HashMap::new(),
            trains: HashMap::new(),
            switches: HashMap::new(),
        }
    }

    pub fn initialize(self) -> Map<MapStateInitialized>{
        todo!()
    }
}