use std::error::Error;
use std::fmt::Display;
use crate::map::Map;
use crate::map::states::MapStateUninitialized;
use anyhow::Result;
pub use crate::map::map_creation_object::*;

pub struct MapCreationView{
    map: Map<MapStateUninitialized>
}

impl MapCreationView{
    pub fn new() -> Self{
        MapCreationView{
            map: Map::new()
        }
    }

    pub fn to_map(self) -> Map<MapStateUninitialized>{
        self.map
    }

    pub fn add_node(&mut self, node: Position) -> Result<()>{
        self.map.add_node(node)
    }
    pub fn add_nodes(&mut self, nodes: &[Position]) -> Result<()>{
        for node in nodes{
            self.map.add_node(*node)?;
        }
        Ok(())
    }
    pub fn add_train(&mut self, train: Train, train_direction: Direction, position: Position) -> Result<()>{
        self.map.add_train(train,train_direction,position).into()
    }
    pub fn add_switch(&mut self, switch: Switch) -> Result<()>{
        self.map.add_switch(switch).into()
    }
    pub fn add_switches(&mut self, switches: &[Switch]) -> Result<()>{
        for switch in switches{
            self.map.add_switch(*switch)?;
        }
        Ok(())
    }
}