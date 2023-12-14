use super::CompleteInitializationMut;
use crate::map::nodes::Link;
use crate::map::Map;
use crate::map::references::{IntiNodeRef, IntiSwitchRef};
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

impl<'a> CompleteInitializationMut<'a> for Link<'a,MapStateInitialized>{
    type InitFromType = Link<'a,MapStateUninitialized>;
    fn complete_initialization(&'a mut self, init_from: Self::InitFromType, map: &'a Map<'a, MapStateInitialized>) {
        self.length = init_from.length;
        self.max_speed = init_from.max_speed;
        self.node = IntiNodeRef{
            node: map.get_node(init_from.node.position)
        };
        self.controller.complete_initialization(init_from.controller, map);
        self.direction = init_from.direction;
    }
}
