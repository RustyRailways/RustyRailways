use std::cell::RefCell;
use std::collections::HashMap;
use common_infrastructure::devices::Switch;
use common_infrastructure::Position;
use common_infrastructure::devices::Train as TrainEnum;
pub mod nodes;
use nodes::{GenericNode, NodeType, RoadNode, SwitchNode};


#[derive(Debug)]
pub struct Map{
    nodes: HashMap<Position, GenericNode>,
    switch_to_position: HashMap<Switch,Position>,
}



impl Map {
    pub fn get_node_at(&self, position: Position) -> &GenericNode{
        return self.nodes.get(&position).unwrap()
    }
}
