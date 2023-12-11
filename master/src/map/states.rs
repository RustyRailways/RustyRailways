// The map has 2 states, uninitialized and initialized
// this distinction is made to easily allow the map to be
// serialized and deserialized in json format...
// An uninitialized map dose not have the internal references
// but can easily deserialized and serialized.
// An initialized map has the internal references, so it's easy
// to use, but can't be serialized and deserialized.

use std::ops::Deref;
use super::{Map, Node};
use serde::{Serialize, Deserialize};
use common_infrastructure::Position;
use common_infrastructure::devices::{Switch, Train};


///////////////////// Definition if the generic states /////////////////////

pub trait State{
    type InitializedType<'a>: InitializedState<'a>;
    type UninitializedType: UninitializedState;
}

pub trait UninitializedState: State + Serialize + Deserialize<'static>{
    fn initialize<'a>(self, map: &'a Map) -> Self::InitializedType<'a>;
}

pub trait InitializedState<'a>: State{
    fn un_initialize(self) -> Self::UninitializedType;
}

///////////////////// Definition of the NodeRef in his two states /////////////////////


pub trait NodeRef: State{}

#[derive(Serialize, Deserialize)]
struct UnIntiNodeRef{
    position: Position
}
impl NodeRef for UnIntiNodeRef{}
impl UninitializedState for UnIntiNodeRef{
    fn initialize(self, map: &Map) -> Self::InitializedType<'_> {
        let node = map.get_node(self.position);
        IntiNodeRef{
            node
        }
    }
}
impl State for UnIntiNodeRef{
    type InitializedType<'a> = IntiNodeRef<'a>;
    type UninitializedType = Self;
}
struct IntiNodeRef<'a>{
    node: &'a Node
}
impl<'a> NodeRef for IntiNodeRef<'a>{}
impl Deref for IntiNodeRef<'_>{
    type Target = Node;
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
impl State for IntiNodeRef<'_>{
    type InitializedType<'b> = IntiNodeRef<'b>;
    type UninitializedType = UnIntiNodeRef;
}