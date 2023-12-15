use super::{CompleteInitializationMut,CompleteInitialization};
use crate::map::nodes::Node;
use crate::map::Map;
use crate::map::references::IntiNodeRef;
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

impl CompleteInitialization for Node<MapStateInitialized>{
    type InitFromType = Node<MapStateUninitialized>;
    fn complete_initialization(&self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>) {
        self.adjacent_nodes.borrow_mut().complete_initialization(&init_from.adjacent_nodes.borrow(), map);
        self.status.borrow_mut().complete_initialization(&init_from.status.borrow(), map);
    }
}
