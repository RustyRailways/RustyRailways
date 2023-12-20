use crate::map::devices::TrainController;
use super::{CompleteInitializationMut, CompleteInitialization, Initialize, UnInitialize};
use crate::map::nodes::{AdjacentNodes, Node, NodeStatus};
use crate::map::Map;
use crate::map::references::IntiNodeRef;
use crate::map::states::{MapStateInitialized, MapStateUninitialized, ReferenceStateInitialized};

impl CompleteInitializationMut for TrainController<MapStateInitialized>{
    type InitFromType = TrainController<MapStateUninitialized>;
    fn complete_initialization(&mut self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>) {
        self.current_position.complete_initialization(&init_from.current_position, map);
    }
}

impl Initialize for TrainController<MapStateUninitialized> {
    type InitializedType = TrainController<MapStateInitialized>;
    fn initialize(self) -> Self::InitializedType {
        TrainController{
            train: self.train,
            current_speed: self.current_speed,
            current_position: unsafe{IntiNodeRef::new_null()},
        }
    }
}
impl UnInitialize for TrainController<MapStateInitialized> {
    type UninitializedType = TrainController<MapStateUninitialized>;
    fn un_initialize(self) -> Self::UninitializedType {
        TrainController{
            train: self.train,
            current_speed: self.current_speed,
            current_position: self.current_position.un_initialize(),
        }
    }
}