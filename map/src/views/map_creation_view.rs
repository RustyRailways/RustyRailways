use std::collections::HashSet;
use crate::Map;
use crate::states::MapStateUninitialized;
use anyhow::Result;
pub use crate::map_creation_object::*;
use crate::constants::{DEFAULT_SWITCH_DISTANCE, DEFAULT_SWITCH_SPEED};
use crate::map_creation_error::MapCreationError;

/// # This struct is used to create a map
pub struct MapCreationView{
    map: Map<MapStateUninitialized>,
    used_switches: HashSet<Switch>,
    has_added_links: bool,
    has_added_trains: bool,
}

impl MapCreationView{
    /// # Create a new empty map
    pub fn new() -> Self{
        MapCreationView{
            map: Map::new(),
            used_switches: HashSet::new(),
            has_added_links: false,
            has_added_trains: false,
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
    /// - can return an error if at least one link is already been added to the map
    /// - can return an error if at least one train is already been added to the map
    pub fn add_switch_station(&mut self,
                              switch: Switch,
                              position_center: Position,
                              position_diverted: Position,
                              position_straight: Position
    ) -> Result<()>{

        if self.has_added_links{
            return Err(MapCreationError::new("all switch_stations must be added before any link is added").into());
        }
        if self.has_added_trains{
            return Err(MapCreationError::new("all switch_stations must be added before any train is added").into());
        }

        if self.used_switches.contains(&switch){
            return Err(MapCreationError::new("Switch already used").into());
        }

        // from main to diverted
        self.map.add_link(position_center, position_diverted,
                          Direction::Forward, Direction::Backward,
                          DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED ,
                          DEFAULT_SWITCH_SPEED,
                          Some((switch, SwitchPosition::Diverted)))?;

        // from main to straight
        self.map.add_link(position_center, position_straight,
                          Direction::Forward, Direction::Backward,
                          DEFAULT_SWITCH_DISTANCE,DEFAULT_SWITCH_SPEED ,
                          DEFAULT_SWITCH_SPEED,
                          Some((switch, SwitchPosition::Straight)))?;

        self.used_switches.insert(switch);

        Ok(())
    }

    /// # add a link between two nodes
    /// ## arguments:
    /// - from: the id of the node where the link starts
    /// - to: the id of the node where the link ends
    /// - max_speed: the maximum speed allowed on the link
    /// - length: the length of the link
    /// ## errors:
    /// - can return an error if the link already exists, or the node is already
    ///   linked to something else
    /// - can return an error if at least one train has already been added to the map
    pub fn add_link(&mut self,
                    n1: Position,
                    n2: Position,
                    max_speed_n1_to_n2: i8,
                    max_speed_n2_to_n1: i8,
                    length: u32,
                    ) -> Result<()>{


        if self.has_added_trains{
            return Err(MapCreationError::new("all links must be added before any train is added").into());
        }

        self.has_added_links = true;

        let n1_ptr = self.map.get_node(n1)?;
        let n2_ptr = self.map.get_node(n2)?;

        let direction_n1 = n1_ptr.get_next_available_direction()?;
        let direction_n2 = n2_ptr.get_next_available_direction()?;

        self.map.add_link(n1, n2, direction_n1, direction_n2, length, max_speed_n1_to_n2,max_speed_n2_to_n1, None).into()
    }

    /// # add one train to the map
    /// ## arguments:
    ///  - train: the id train to add
    ///  - position: the id of the node where the train is located
    ///  - pointing_to: the id of the node where the train is pointing to,
    ///    this means that if pointing_to is P2, for examole, the train will reach P2 if the speed
    ///    is positive
    ///    if pointing_to is None, the train will point towards the dead end of the link, and will
    ///    derail if the speed is positive
    /// ## errors:
    ///  - can return an error if the train already exists
    ///  - can return an error if the node does not exist
    ///  - can return an error if the node is already occupied by another train
    ///  - can return an error if pointing_to is not None and the node is not in the adjacent nodes
    ///  - can return an error if pointing_to is None and the node does not have a dead end
    pub fn add_train(&mut self, train: Train, position: Position, pointing_to: Option<Position>) -> Result<()>{
        self.has_added_trains = true;
        let node = self.map.get_node(position)?;
        let mut direction = None;

        // direction is some, so i need to find the direction that points to the node
        if let Some(pointing_to) = pointing_to{
            for link in node.adjacent_nodes.borrow().get_adjacent_nodes(){
                if link.node.position == pointing_to{
                    direction = Some(link.direction);
                }
            }
        // direction is none, so i need to point the train to the dead end
        }else if let Ok(dead_end_direction) = node.get_next_available_direction(){
            direction = Some(dead_end_direction);
        }

        let direction = match direction {
            Some(v)=> v,
            None => return Err(MapCreationError::new("The pointing_to has not be found").into())
        };
        self.map.add_train(train,direction,position).into()
    }
}