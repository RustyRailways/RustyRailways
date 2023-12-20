use crate::map::{Map,MapStateInitialized};
use crate::map::states::{ReferenceStateInitialized, ReferenceState, ReferenceStateUninitialized};
use crate::map::nodes::Node;
use common_infrastructure::Position;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub trait NodeRef: ReferenceState {}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct UnIntiNodeRef{
    pub position: Position
}
impl NodeRef for UnIntiNodeRef{}
impl ReferenceStateUninitialized for UnIntiNodeRef{
    fn initialize(self, map: & Map<MapStateInitialized>) -> Self::InitializedType {
        let node = map.get_node(self.position);
        IntiNodeRef{
            node
        }
    }
}
impl ReferenceState for UnIntiNodeRef{
    type InitializedType = IntiNodeRef;
    type UninitializedType = Self;
}

#[derive(Debug,Clone)]
pub struct IntiNodeRef{
    pub node:  *const Node<MapStateInitialized>
}

impl IntiNodeRef{

    // allow to construct a new pointer that is null, that can be initialized later
    pub unsafe fn new_null() -> Self{
        IntiNodeRef{
            node: std::ptr::null()
        }
    }

    pub fn complete_initialization(&mut self, init_from: &UnIntiNodeRef , map: &Map<MapStateInitialized>){
        *self = init_from.initialize(map);
    }
}

impl NodeRef for IntiNodeRef{}
impl Deref for IntiNodeRef{
    type Target = Node<MapStateInitialized>;
    fn deref(&self) -> &Self::Target {
        unsafe {&*self.node}
    }
}
impl ReferenceStateInitialized for IntiNodeRef{
    fn un_initialize(self) -> Self::UninitializedType {
        UnIntiNodeRef{
            position: self.position
        }
    }
}
impl ReferenceState for IntiNodeRef{
    type InitializedType = IntiNodeRef;
    type UninitializedType = UnIntiNodeRef;
}