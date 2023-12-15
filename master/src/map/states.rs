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
use std::marker::PhantomData;

pub trait ReferenceState {
    type InitializedType<'a>: ReferenceStateInitialized<'a>;
    type UninitializedType: ReferenceStateUninitialized;
}

pub trait ReferenceStateUninitialized: ReferenceState + Serialize + Deserialize<'static>{
    fn initialize<'a>(self, map: &'a Map<'a, MapStateInitialized>) -> Self::InitializedType<'a>;
}

pub trait ReferenceStateInitialized<'a>: ReferenceState {
    fn un_initialize(self) -> Self::UninitializedType;
}

pub trait MapState<'a>{
    type NodeRefType: NodeRef + Debug + Serialize;
    type TrainRefType: TrainRef + Debug + Serialize;
    type SwitchRefType: SwitchRef + Debug + Serialize;
}

pub struct MapStateUninitialized{}
impl<'a> MapState<'a> for MapStateUninitialized{
    type NodeRefType = UnIntiNodeRef;
    type TrainRefType = UnIntiTrainRef;
    type SwitchRefType = UnIntiSwitchRef;
}
#[derive(Debug,Serialize,Deserialize)]
pub struct MapStateInitialized{}

impl<'a> MapState<'a> for MapStateInitialized{
    type NodeRefType = IntiNodeRef<'a>;
    type TrainRefType = IntiTrainRef<'a>;
    type SwitchRefType = IntiSwitchRef<'a>;
}