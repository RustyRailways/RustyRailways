use crate::map::devices::TrainController;
use super::{CompleteInitialization, Initialize, UnInitialize};
use crate::map::Map;
use crate::map::references::IntiNodeRef;
use crate::map::states::{MapStateInitialized, MapStateUninitialized, ReferenceStateInitialized};

impl CompleteInitialization for TrainController<MapStateInitialized>{
    type InitFromType = TrainController<MapStateUninitialized>;
    fn complete_initialization(&self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>) {
        self.current_position.borrow_mut().complete_initialization(&init_from.current_position.borrow(), map);
    }
}

impl Initialize for TrainController<MapStateUninitialized> {
    type InitializedType = TrainController<MapStateInitialized>;
    fn initialize(self) -> Self::InitializedType {
        TrainController{
            direction: self.direction,
            train: self.train,
            current_speed: self.current_speed,
            current_position: unsafe{IntiNodeRef::new_null().into()},
            status: self.status,
        }
    }
}
impl UnInitialize for TrainController<MapStateInitialized> {
    type UninitializedType = TrainController<MapStateUninitialized>;
    fn un_initialize(self) -> Self::UninitializedType {
        TrainController{
            direction: self.direction,
            train: self.train,
            current_speed: self.current_speed,
            current_position: self.current_position.into_inner().un_initialize().into(),
            status: self.status,
        }
    }
}