use crate::map::{Map,MapStateInitialized};
use crate::map::states::{InitializedState, ReferenceState, UninitializedState};
use crate::map::nodes::Node;
use common_infrastructure::Position;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub trait NodeRef: ReferenceState {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct UnIntiNodeRef{
    position: Position
}
impl NodeRef for UnIntiNodeRef{}
impl UninitializedState for UnIntiNodeRef{
    fn initialize<'a>(self, map: &'a Map<'a, MapStateInitialized>) -> Self::InitializedType<'a> {
        let node = map.get_node(self.position);
        IntiNodeRef{
            node
        }
    }
}
impl ReferenceState for UnIntiNodeRef{
    type InitializedType<'a> = IntiNodeRef<'a>;
    type UninitializedType = Self;
}


pub struct IntiNodeRef<'a>{
    node: &'a Node<'a,MapStateInitialized>
}
impl<'a> NodeRef for IntiNodeRef<'a>{}
impl<'a> Deref for IntiNodeRef<'a>{
    type Target = Node<'a,MapStateInitialized>;
    fn deref(&self) -> &Self::Target {
        self.node
    }
}
impl<'a> InitializedState<'a> for IntiNodeRef<'a>{
    fn un_initialize(self) -> Self::UninitializedType {
        UnIntiNodeRef{
            position: self.position
        }
    }
}
impl ReferenceState for IntiNodeRef<'_>{
    type InitializedType<'b> = IntiNodeRef<'b>;
    type UninitializedType = UnIntiNodeRef;
}