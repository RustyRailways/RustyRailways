use std::collections::HashSet;
use crate::map::Map;
use crate::map::states::MapStateUninitialized;
use anyhow::Result;
pub use crate::map::map_creation_object::*;
use crate::constants::{DEFAULT_SWITCH_DISTANCE, DEFAULT_SWITCH_SPEED};
use crate::map::map_creation_error::MapCreationError;

pub struct MapCreationView{
    map: Map<MapStateUninitialized>,
    used_switches: HashSet<Switch>,
    has_added_links: bool
}

impl MapCreationView{
    /// # Create a new empty map
    pub fn new() -> Self{
        MapCreationView{
            map: Map::new(),
            used_switches: HashSet::new(),
            has_added_links: false
        }
    }

    /// # return an uninitialized map
    pub fn to_map(self) -> Map<MapStateUninitialized>{
        self.map
    }

    /// # add one node to the map
    /// ## errors
    ///  - can returns an error if the node already exists
    pub fn add_node(&mut self, node: Position) -> Result<()>{
        self.map.add_node(node)
    }

    /// # add a list of nodes to the map
    /// ## errors
    ///  - can returns an error if at least one of the nodes already exists
    pub fn add_nodes(&mut self, nodes: &[Position]) -> Result<()>{
        for node in nodes{
            self.map.add_node(*node)?;
        }
        Ok(())
    }

    /// # add one train to the map
    /// ## arguments:
    ///  - train: the id train to add
    ///  - train_direction: the direction the train is facing, can be either Forward or Backward.
    ///     - Forward means that if the train speed is positive, it will move forward (according to the orientation of the node the train is on)
    ///     - Backward means that if the train speed is positive, it will move backward (according to the orientation of the node the train is on)
    ///
    ///    if the train speed is negative, the train will move in the opposite direction
    ///  - position: the id of the node where the train is located
    /// ## errors:
    ///  - can return an error if the train already exists
    ///  - can return an error if the node does not exist
    ///  - can return an error if the node is already occupied by another train
    pub fn add_train(&mut self, train: Train, train_direction: Direction, position: Position) -> Result<()>{
        self.map.add_train(train,train_direction,position).into()
    }

    /// # add a switch to the map
    /// ## errors:
    /// - can return an error if the switch already exists
    pub fn add_switch(&mut self, switch: Switch) -> Result<()>{
        self.map.add_switch(switch).into()
    }

    /// # add a list of switches to the map
    /// ## errors:
    /// - can return an error if at least one of the switches already exists
    pub fn add_switches(&mut self, switches: &[Switch]) -> Result<()>{
        for switch in switches{
            self.map.add_switch(*switch)?;
        }
        Ok(())
    }

    /// # add switch station, aka 2 links between 3 nodes with the switch
    /// ## arguments:
    ///  - switch: the id of the switch
    ///  - position_center: the id of the node that can be reached from the two other nodes
    ///  - position_diverted: the id of the node that can be reached from position_center when the switch is diverted
    ///  - position_straight: the id of the node that can be reached from position_center when the switch is straight
    /// ## errors:
    /// - can return an error if the switch does not exist
    /// - can return an error if at least one of the nodes does not exist
    /// - can return an error if one of the nodes is already linked to something else
    /// - can return an error if the switch is already used
    /// - can return an error if at least one link is already added to the map
    pub fn add_switch_station(&mut self,
                              switch: Switch,
                              position_center: Position,
                              position_diverted: Position,
                              position_straight: Position
    ) -> Result<()>{

        if self.has_added_links{
            return Err(MapCreationError::new("all switch_stations must be added before any link is added").into());
        }

        if self.used_switches.contains(&switch){
            return Err(MapCreationError::new("Switch already used").into());
        }

        // from main to diverted
        self.map.add_link(position_center, position_diverted,
                          Direction::Forward, Direction::Backward,
                          DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED ,
                          Some((switch, SwitchPosition::Diverted)))?;

        // from main to straight
        self.map.add_link(position_center, position_straight,
                          Direction::Forward, Direction::Backward,
                          DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED ,
                          Some((switch, SwitchPosition::Diverted)))?;

        self.used_switches.insert(switch);

        Ok(())
    }
}