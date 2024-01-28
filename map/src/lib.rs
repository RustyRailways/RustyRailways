use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use common_infrastructure::Position;
use common_infrastructure::devices::{Switch, Train};
pub use crate::states::{MapState, MapStateInitialized, MapStateUninitialized};
use devices::{SwitchController, TrainController};
use nodes::Node;
use references::{UnIntiNodeRef, UnIntiSwitchRef, UnIntiTrainRef};
use crate::devices::SwitchControllerOption;
use crate::nodes::Direction;
use map_creation_object::SwitchPosition;
use anyhow::Result;
use crate::map_creation_error::MapCreationError;

pub mod constants;
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
pub mod map_comunication;
#[cfg(test)]
pub(crate) mod map_tests;

#[derive(Debug,Serialize,Deserialize)]
pub struct Map<T: MapState>{
    nodes: HashMap<Position, Node<T>>,
    trains: HashMap<Train, TrainController<T>>,
    switches: HashMap<Switch, SwitchController>,
    #[serde(default)]
    #[serde(skip)]
    comunciator: Option<map_comunication::MapComunicationMaster>,
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

impl Map<MapStateInitialized>{
    pub fn get_node_mut(&mut self, position: Position) -> Result<&mut Node<MapStateInitialized>>{
        self.nodes.get_mut(&position).ok_or(MapCreationError::new("Node does not exist").into())
    }
    pub fn get_train_mut(&mut self, train: Train) -> Result<&mut TrainController<MapStateInitialized>>{
        self.trains.get_mut(&train).ok_or(MapCreationError::new("Train does not exist").into())
    }
    pub fn get_switch_mut(&mut self, switch: Switch) -> Result<&mut SwitchController>{
        self.switches.get_mut(&switch).ok_or(MapCreationError::new("Switch does not exist").into())
    }
    pub fn add_comunicator(&mut self, mut comunciator: map_comunication::MapComunicationMaster){
        for (train, train_controller) in &self.trains{
            comunciator.set_train_position(*train, train_controller.get_position());
            comunciator.set_train_speed(*train, train_controller.current_speed);
            comunciator.set_train_status(*train, train_controller.status)
        }
        for (switch, switch_controller) in &self.switches{
            let pos = switch_controller.position.borrow().clone();
            comunciator.set_switch(*switch, pos);
        }
        self.comunciator = Some(comunciator);
    }
}

impl Map<MapStateUninitialized>{
    pub fn new() -> Self{
        Map{
            nodes: HashMap::new(),
            trains: HashMap::new(),
            switches: HashMap::new(),
            comunciator: None,
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

    pub fn add_link(&mut self, position_1: Position, position_2: Position,
                    direction_from: Direction, direction_to: Direction,
                    length: u32, max_speed_1_to_2: i8, max_speed_2_to_1: i8, switch: Option<(Switch,SwitchPosition)>
    ) -> Result<()>{

        if let Some(switch) = &switch{
            if !self.switches.contains_key(&switch.0){
                return Err(MapCreationError::new("Switch does not exist").into());
            }
        }

        let node_1: Result<_,anyhow::Error> = self.get_node(position_1).map_err(
            |_| MapCreationError::new("Node from does not exist").into()
        );
        let node_1 = node_1?;

        let node_2: Result<_,anyhow::Error>  = self.get_node(position_2).map_err(
            |_| MapCreationError::new("Node to does not exist").into()
        );
        let node_2 = node_2?;

        let node_1_ref = UnIntiNodeRef{
            position: position_1,
        };

        let node_2_ref = UnIntiNodeRef{
            position: position_2,
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

        node_1.add_link(node_2_ref, direction_from, max_speed_1_to_2, length, switch.clone())?;
        node_2.add_link(node_1_ref, direction_to, max_speed_2_to_1, length, switch)?;

        Ok(())
    }
}

pub mod map_creation_object{
    pub use crate::MapStateInitialized;
    pub use crate::MapStateUninitialized;
    pub use common_infrastructure::Position;
    pub use common_infrastructure::devices::{Switch, Train};
    pub use crate::nodes::Direction;
    pub use crate::devices::SwitchPosition;
    pub use crate::devices::TrainStatus;
}
