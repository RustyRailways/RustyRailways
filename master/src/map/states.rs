// The map has 2 states, uninitialized and initialized
// this distinction is made to easily allow the map to be
// serialized and deserialized in json format...
// An uninitialized map dose not have the internal references
// but can easily deserialized and serialized.
// An initialized map has the internal references, so it's easy
// to use, but can't be serialized and deserialized.

use serde::{Serialize, Deserialize};
use crate::map::Map;
use crate::map::references::*;
use std::marker::PhantomData;

pub trait ReferenceState {
    type InitializedType<'a>: InitializedState<'a>;
    type UninitializedType: UninitializedState;
}

pub trait UninitializedState: ReferenceState + Serialize + Deserialize<'static>{
    fn initialize<'a>(self, map: &'a Map<MapStateInitialized<'a>>) -> Self::InitializedType<'a>;
}

pub trait InitializedState<'a>: ReferenceState {
    fn un_initialize(self) -> Self::UninitializedType;
}

pub trait MapState{
    type NodeRefType: NodeRef;
    type TrainRefType: TrainRef;
    type SwitchRefType: SwitchRef;
}

pub struct MapStateUninitialized{}
impl MapState for MapStateUninitialized{
    type NodeRefType = UnIntiNodeRef;
    type TrainRefType = UnIntiTrainRef;
    type SwitchRefType = UnIntiSwitchRef;
}
pub struct MapStateInitialized<'a>{
    lifetime : PhantomData<&'a Map<MapStateInitialized<'a>>>
}
impl<'a> MapState for MapStateInitialized<'a>{
    type NodeRefType = IntiNodeRef<'a>;
    type TrainRefType = IntiTrainRef<'a>;
    type SwitchRefType = IntiSwitchRef<'a>;
}