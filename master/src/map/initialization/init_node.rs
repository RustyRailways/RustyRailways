use super::{CompleteInitializationMut,CompleteInitialization, Initialize, UnInitialize};
use crate::map::nodes::{AdjacentNodes, Node, NodeStatus};
use crate::map::Map;
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

impl CompleteInitialization for Node<MapStateInitialized>{
    type InitFromType = Node<MapStateUninitialized>;
    fn complete_initialization(&self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>) {
        self.adjacent_nodes.borrow_mut().complete_initialization(&init_from.adjacent_nodes.borrow(), map);
        self.status.borrow_mut().complete_initialization(&init_from.status.borrow(), map);
    }
}

impl Initialize for Node<MapStateUninitialized> {
    type InitializedType = Node<MapStateInitialized>;
    fn initialize(self) -> Self::InitializedType {
        Node{
            position: self.position,
            adjacent_nodes: AdjacentNodes::None.into(),
            status: NodeStatus::Unlocked.into(),
        }
    }
}
impl UnInitialize for Node<MapStateInitialized> {
    type UninitializedType = Node<MapStateUninitialized>;
    fn un_initialize(self) -> Self::UninitializedType {
        unimplemented!();/*
        Node{
            position: self.position,
            adjacent_nodes: self.adjacent_nodes.into_inner().un_initialize().into(),
            status: self.status.into_inner().un_initialize().into()
        }*/
    }
}