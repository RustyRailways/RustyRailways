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
use references::{IntiNodeRef, IntiSwitchRef, IntiTrainRef, UnIntiNodeRef, UnIntiSwitchRef, UnIntiTrainRef};
use crate::map::devices::SwitchControllerOption;
use crate::map::nodes::Direction;
use map_creation_object::SwitchPosition;

pub mod states;
pub mod references;

pub mod nodes;

/// Contains Switch and Trains
pub mod devices;
pub mod initialization;
pub mod serde_impls;
#[cfg(test)]
mod map_tests;

#[derive(Debug,Serialize,Deserialize)]
pub struct Map<T: MapState>{
    nodes: HashMap<Position, Node<T>>,
    trains: HashMap<Train, TrainController<T>>,
    switches: HashMap<Switch, SwitchController>,
}

impl<T: MapState> Map<T> {
    pub fn get_node(&self, position: Position) -> &Node<T>{
        self.nodes.get(&position).unwrap()
    }
    pub fn get_train(&self, train: Train) -> &TrainController<T>{
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

    pub fn add_train(&mut self, train: Train, train_direction: Direction, position: Position) -> Result<(),&str>{
        if self.trains.contains_key(&train){
            return Err("Train already exists");
        }
        if !self.nodes.contains_key(&position){
            return Err("Position does not exist");
        }

        self.trains.insert(train, TrainController::new(train,train_direction));
        self.get_node(position).set_train(UnIntiTrainRef{train})?;

        Ok(())
    }

    pub fn add_switch(&mut self, switch: Switch) -> Result<(),&str>{
        if self.switches.contains_key(&switch){
            return Err("Switch already exists");
        }

        self.switches.insert(switch, SwitchController::new(switch));
        Ok(())
    }

    pub fn add_node(&mut self, position: Position) -> Result<(),&str>{
        if self.nodes.contains_key(&position){
            return Err("Node already exists");
        }

        self.nodes.insert(position, Node::new(position));
        Ok(())
    }

    pub fn add_link(&mut self, position_from: Position, position_to: Position,
                    direction_from: Direction, direction_to: Direction,
                    length: u32, max_speed: u32, switch: Option<(Switch,SwitchPosition)>
    ) -> Result<(),&str>{
        if !self.nodes.contains_key(&position_from){
            return Err("Node from does not exist");
        }
        if !self.nodes.contains_key(&position_to){
            return Err("Node to does not exist");
        }
        if let Some(switch) = &switch{
            if !self.switches.contains_key(&switch.0){
                return Err("Switch does not exist");
            }
        }

        let node_from = self.get_node(position_from);
        let node_to = self.get_node(position_to);

        let node_from_ref = UnIntiNodeRef{
            position: position_from,
        };

        let node_to_ref = UnIntiNodeRef{
            position: position_to,
        };

        let switch = match switch {
            None => SwitchControllerOption::<MapStateUninitialized>::NoSwitch,
            Some((switch, position)) => {
                match position {
                    SwitchPosition::Straight => {
                        SwitchControllerOption::SwitchToSetStraight(UnIntiSwitchRef{
                            switch
                        })
                    },
                    SwitchPosition::Diverted => {
                        SwitchControllerOption::SwitchToSetDiverted(UnIntiSwitchRef{
                            switch
                        })
                    },
                }
            }
        };

        node_from.add_link(node_to_ref, direction_from, length, max_speed, switch.clone())?;
        node_to.add_link(node_from_ref, direction_to, length, max_speed, switch)?;

        Ok(())
    }
}

pub mod map_creation_object{
    pub enum SwitchPosition{
        Straight,
        Diverted,
    }

    pub use common_infrastructure::Position;
    pub use common_infrastructure::devices::{Switch, Train};
    pub use crate::map::nodes::Direction;
}

