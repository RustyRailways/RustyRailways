use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Deref;
use common_infrastructure::Position;
use common_infrastructure::devices::{Switch, Train};
use crate::map::states::{NodeRef, SwitchRef, TrainRef, UnIntiNodeRef, IntiNodeRef, UnIntiTrainRef, IntiTrainRef, UnIntiSwitchRef, IntiSwitchRef};

pub mod states;
pub mod references;

pub trait MapState{
    type NodeRefType: NodeRef;
    type TrainRefType: TrainRef;
    type SwitchRefTyp: SwitchRef;
}

pub struct MapStateUninitialized{}
impl MapState for MapStateUninitialized{
    type NodeRefType = UnIntiNodeRef;
    type TrainRefType = UnIntiTrainRef;
    type SwitchRefTyp = UnIntiSwitchRef;
}
pub struct MapStateInitialized<'a>{
    r: PhantomData<&'a Map<MapStateInitialized<'a>>>
    //map: &'a >>
}
impl<'a> MapState for MapStateInitialized<'a>{
    type NodeRefType = IntiNodeRef<'a>;
    type TrainRefType = IntiTrainRef<'a>;
    type SwitchRefTyp = IntiSwitchRef<'a>;
}

pub struct Map<T: MapState>{
    state: PhantomData<T>,
    nodes: HashMap<Position, Node<T>>,
    trains: HashMap<Train, TrainController<T>>,
    switches: HashMap<Switch, SwitchController<T>>,
}

impl<T: MapState> Map<T> {
    pub fn get_node(&self, position: Position) -> &Node<T>{
        self.nodes.get(&position).unwrap()
    }
    pub fn get_train(&self, train: Train) -> &TrainController<T>{
        self.trains.get(&train).unwrap()
    }
    pub fn get_switch(&self, switch: Switch) -> &SwitchController<T>{
        self.switches.get(&switch).unwrap()
    }
}

pub struct Node<T: MapState>{
    state: PhantomData<T>,
    position: Position
}

pub struct TrainController<T: MapState>{
    state: PhantomData<T>,
    train: Train,
}

pub struct SwitchController<T: MapState>{
    state: PhantomData<T>,
    switch: Switch,
}