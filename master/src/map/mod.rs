use std::collections::HashMap;
use std::ops::Deref;
use serde::{Deserialize, Serialize};
use common_infrastructure::Position;
use common_infrastructure::devices::{Switch, Train};
use crate::map::states::{MapState, MapStateInitialized, MapStateUninitialized};
use devices::{SwitchController, TrainController};
use nodes::Node;
use references::{UnIntiNodeRef, UnIntiSwitchRef, UnIntiTrainRef};
use crate::map::devices::SwitchControllerOption;
use crate::map::nodes::Direction;
use map_creation_object::SwitchPosition;
use anyhow::Result;
use crate::map::map_creation_error::MapCreationError;

pub mod states;
pub mod references;
pub mod nodes;
#[allow(dead_code)]
pub mod views;
pub mod map_creation_error;

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
    pub fn get_node(&self, position: Position) -> Result<&Node<T>>{
        self.nodes.get(&position).ok_or(MapCreationError::new("Node does not exist").into())
    }
    pub fn get_train(&self, train: Train) -> Result<&TrainController<T>>{
        self.trains.get(&train).ok_or(MapCreationError::new("Train does not exist").into())
    }
    pub fn get_switch(&self, switch: Switch) -> Result<&SwitchController>{
        self.switches.get(&switch).ok_or(MapCreationError::new("Switch does not exist").into())
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

    pub fn add_train(&mut self, train: Train, train_direction: Direction, position: Position) -> Result<()>{
        if self.trains.contains_key(&train){
            return Err(MapCreationError::new("Train already exists").into());
        }

        self.trains.insert(train, TrainController::new(train,train_direction,position));
        let node = self.get_node(position).map_err(
            |_| <MapCreationError as Into<anyhow::Error>>::into(MapCreationError::new("Node does not exist"))
        )?;
        node.set_train(UnIntiTrainRef{train})?;

        Ok(())
    }

    pub fn add_switch(&mut self, switch: Switch) -> Result<()>{
        if self.switches.contains_key(&switch){
            return Err(MapCreationError::new("Switch already exists").into());
        }

        self.switches.insert(switch, SwitchController::new(switch));
        Ok(())
    }

    pub fn add_node(&mut self, position: Position) -> Result<()>{
        if self.nodes.contains_key(&position){
            return Err(MapCreationError::new("Node already exists").into());
        }

        self.nodes.insert(position, Node::new(position));
        Ok(())
    }

    pub fn add_link(&mut self, position_from: Position, position_to: Position,
                    direction_from: Direction, direction_to: Direction,
                    length: u32, max_speed: i8, switch: Option<(Switch,SwitchPosition)>
    ) -> Result<()>{

        if let Some(switch) = &switch{
            if !self.switches.contains_key(&switch.0){
                return Err(MapCreationError::new("Switch does not exist").into());
            }
        }

        let node_from: Result<_,anyhow::Error> = self.get_node(position_from).map_err(
            |_| MapCreationError::new("Node from does not exist").into()
        );
        let node_from = node_from?;

        let node_to: Result<_,anyhow::Error>  = self.get_node(position_to).map_err(
            |_| MapCreationError::new("Node to does not exist").into()
        );
        let node_to = node_to?;

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

        node_from.add_link(node_to_ref, direction_from, max_speed,length, switch.clone())?;
        node_to.add_link(node_from_ref, direction_to, max_speed,length, switch)?;

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

