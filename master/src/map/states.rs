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
use crate::map::{MapStateInitialized, SwitchController, TrainController};


///////////////////// Definition if the generic states /////////////////////

pub trait State{
    type InitializedType<'a>: InitializedState<'a>;
    type UninitializedType: UninitializedState;
}

pub trait UninitializedState: State + Serialize + Deserialize<'static>{
    fn initialize<'a>(self, map: &'a Map<MapStateInitialized<'a>>) -> Self::InitializedType<'a>;
}

pub trait InitializedState<'a>: State{
    fn un_initialize(self) -> Self::UninitializedType;
}

///////////////////// Definition of the NodeRef in his two states ////////////////////////


pub trait NodeRef: State{}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct UnIntiNodeRef{
    position: Position
}
impl NodeRef for UnIntiNodeRef{}
impl UninitializedState for UnIntiNodeRef{
    fn initialize<'a>(self, map: &'a Map<MapStateInitialized<'a>>) -> Self::InitializedType<'a> {
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


pub struct IntiNodeRef<'a>{
    node: &'a Node<MapStateInitialized<'a>>
}
impl<'a> NodeRef for IntiNodeRef<'a>{}
impl<'a> Deref for IntiNodeRef<'a>{
    type Target = Node<MapStateInitialized<'a>>;
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


///////////////////// Definition of the TrainRef in his two states ////////////////////////

pub trait TrainRef: State{}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct UnIntiTrainRef{
    train: Train
}
impl TrainRef for UnIntiTrainRef{}

impl UninitializedState for UnIntiTrainRef{
    fn initialize<'a>(self, map: &'a Map<MapStateInitialized<'a>>) -> Self::InitializedType<'a> {
        let train = map.get_train(self.train);
        IntiTrainRef{
            train
        }
    }

}
impl State for UnIntiTrainRef{
    type InitializedType<'a> = IntiTrainRef<'a>;
    type UninitializedType = Self;
}

pub struct IntiTrainRef<'a>{
    train: &'a TrainController<MapStateInitialized<'a>>
}

impl TrainRef for IntiTrainRef<'_>{}

impl<'a> Deref for IntiTrainRef<'a>{
    type Target = TrainController<MapStateInitialized<'a>>;
    fn deref(&self) -> &Self::Target {
        self.train
    }
}
impl InitializedState<'_> for IntiTrainRef<'_>{
    fn un_initialize(self) -> Self::UninitializedType {
        UnIntiTrainRef{
            train: self.train.train
        }
    }
}
impl State for IntiTrainRef<'_>{
    type InitializedType<'a> = Self;
    type UninitializedType = UnIntiTrainRef;
}


///////////////////// Definition of the SwitchRef in his two states ////////////////////////

pub trait SwitchRef: State{}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct UnIntiSwitchRef{
    switch: Switch
}
impl SwitchRef for UnIntiSwitchRef{}

impl UninitializedState for UnIntiSwitchRef{
    fn initialize<'a>(self, map: &'a Map<MapStateInitialized<'a>>) -> Self::InitializedType<'a> {
        let switch = map.get_switch(self.switch);
        IntiSwitchRef{
            switch
        }
    }

}
impl State for UnIntiSwitchRef{
    type InitializedType<'a> = IntiSwitchRef<'a>;
    type UninitializedType = Self;
}

pub struct IntiSwitchRef<'a>{
    switch: &'a SwitchController<MapStateInitialized<'a>>
}

impl SwitchRef for IntiSwitchRef<'_>{}

impl<'a> Deref for IntiSwitchRef<'a>{
    type Target = SwitchController<MapStateInitialized<'a>>;
    fn deref(&self) -> &Self::Target {
        self.switch
    }
}
impl InitializedState<'_> for IntiSwitchRef<'_>{
    fn un_initialize(self) -> Self::UninitializedType {
        UnIntiSwitchRef{
            switch: self.switch.switch
        }
    }
}
impl State for IntiSwitchRef<'_>{
    type InitializedType<'a> = Self;
    type UninitializedType = UnIntiSwitchRef;
}
