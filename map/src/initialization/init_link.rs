use super::CompleteInitializationMut;
use crate::nodes::Link;
use crate::Map;
use crate::references::IntiNodeRef;
use crate::states::{MapStateInitialized, MapStateUninitialized};

impl CompleteInitializationMut for Link<MapStateInitialized>{
    type InitFromType = Link<MapStateUninitialized>;
    fn complete_initialization(& mut self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>) {
        self.length = init_from.length;
        self.max_speed = init_from.max_speed;
        self.node = IntiNodeRef{
            node: map.get_node(init_from.node.position).unwrap()
        };
        self.controller.complete_initialization(&init_from.controller, map);
        self.direction = init_from.direction;
    }
}
