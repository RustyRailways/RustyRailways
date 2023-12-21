// The map has 2 states, uninitialized and initialized
// this distinction is made to easily allow the map to be
// serialized and deserialized in json format...
// An uninitialized map dose not have the internal references
// but can easily deserialized and serialized.
// An initialized map has the internal references, so it's easy
// to use, but can't be serialized and deserialized.

use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::map::Map;
use crate::map::references::*;

pub trait ReferenceState {
    type InitializedType: ReferenceStateInitialized;

    type UninitializedType: ReferenceStateUninitialized;
}

pub trait ReferenceStateUninitialized: ReferenceState + Serialize + Deserialize<'static>{
    fn initialize(self, map: & Map<MapStateInitialized>) -> Self::InitializedType;
}

pub trait ReferenceStateInitialized: ReferenceState {
    fn un_initialize(self) -> Self::UninitializedType;
}


pub trait MapState{
    type NodeRefType: NodeRef + Debug + Serialize + for<'a> Deserialize<'a> + Clone;
    type TrainRefType: TrainRef + Debug + Serialize + for<'a> Deserialize<'a> + Clone;
    type SwitchRefType: SwitchRef + Debug + Serialize + for<'a> Deserialize<'a> + Clone;
}

#[derive(Debug,Serialize,Deserialize,Clone,Eq, PartialEq)]
pub struct MapStateUninitialized{}
impl MapState for MapStateUninitialized{
    type NodeRefType = UnIntiNodeRef;
    type TrainRefType = UnIntiTrainRef;
    type SwitchRefType = UnIntiSwitchRef;
}
#[derive(Debug,Serialize,Deserialize)]
pub struct MapStateInitialized{}
impl MapState for MapStateInitialized{
    type NodeRefType = IntiNodeRef;
    type TrainRefType = IntiTrainRef;
    type SwitchRefType = IntiSwitchRef;
}