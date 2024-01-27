use std::cell::RefCell;
use std::rc::Rc;
use common_infrastructure::devices::{Switch, Train};
use common_infrastructure::Position;
use crate::Map;
use anyhow::Result;
use crate::devices::TrainStatus;
use crate::initialization::UnInitialize;
use crate::map_creation_object::SwitchPosition;
use crate::nodes::NodeStatus;
use crate::states::{MapStateInitialized, MapStateUninitialized};

pub struct MapVisualizationView{
    map: Rc<RefCell<Map<MapStateInitialized>>>
}
/// This view is used to visualize the map
/// can give information about position of trains and status of switches and nodes.
impl MapVisualizationView{
    /// # Creates a new MapVisualizationView
    pub fn new(map: Rc<RefCell<Map<MapStateInitialized>>>) -> Self{
        MapVisualizationView{ map }
    }

    /// # Returns the position of a train
    /// # Errors
    /// Returns an error if the train does not exist
    pub fn get_train_position(&self, train: Train) -> Result<Position>{
        let pos = self.map.borrow().get_train(train)?.get_position();
        Ok(pos)
    }

    /// # Returns the speed of a train
    /// # Errors
    /// Returns an error if the train does not exist
    pub fn get_train_speed(&self, train: Train) -> Result<i8>{
        let speed = self.map.borrow().get_train(train)?.current_speed;
        Ok(speed)
    }

    /// # Returns position of a switch
    /// # Errors
    /// Returns an error if the switch does not exist
    pub fn get_switch_status(&self, switch: Switch) -> Result<SwitchPosition>{
        let position = self.map.borrow().get_switch(switch)?.position.borrow().clone();
        Ok(position)
    }

    /// # Returns the status of a node
    /// # Errors
    /// Returns an error if the node does not exist
    pub fn get_node_status(&self, position: Position) -> Result<NodeStatus<MapStateUninitialized>>{
        let status = self.map.borrow().get_node(position)?.status.borrow().clone();
        Ok(status.un_initialize())
    }

    pub fn get_train_status(&self, train: Train) -> Result<TrainStatus>{
        let status = self.map.borrow().get_train(train)?.status.clone();
        Ok(status)
    }
}