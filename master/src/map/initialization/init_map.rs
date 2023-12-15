use std::collections::HashMap;
use crate::map::initialization::{CompleteInitialization, Initialize, UnInitialize};
use crate::map::Map;
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

impl Initialize for Map<MapStateUninitialized>{
    type InitializedType = Map<MapStateInitialized>;
    fn initialize(self) -> Self::InitializedType {
        let mut nodes = HashMap::new();
        for (position, node) in &self.nodes{
            nodes.insert(*position, node.clone().initialize());
        }
        let mut trains = HashMap::new();
        for (train, train_controller) in &self.trains{
            trains.insert(*train, train_controller.clone());
        }
        let mut switches = HashMap::new();
        for (switch, switch_controller) in &self.switches{
            switches.insert(*switch, switch_controller.clone());
        }
        let new_map = Map{
            nodes,
            trains,
            switches,
        };
        new_map.nodes.values().for_each(|node| node.complete_initialization(self.get_node(node.position), &new_map));
        new_map
    }
}