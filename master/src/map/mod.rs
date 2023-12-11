use std::collections::HashMap;
use std::ops::Deref;
use common_infrastructure::Position;
use common_infrastructure::devices::{Switch, Train};

pub mod states;


pub struct Map{
    nodes: HashMap<Position, Node>,
    trains: HashMap<Train, Position>
}

impl Map {
    pub fn get_node(&self, position: Position) -> &Node{
        self.nodes.get(&position).unwrap()
    }
}

pub struct Node{
    position: Position
}

pub struct TrainController{
}

pub struct SwitchController{

}