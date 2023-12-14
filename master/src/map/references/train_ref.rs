use crate::map::{Map,MapStateInitialized, TrainController};
use crate::map::states::{InitializedState, ReferenceState, UninitializedState};
use common_infrastructure::devices::Train;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub trait TrainRef: ReferenceState {}

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
impl ReferenceState for UnIntiTrainRef{
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
impl ReferenceState for IntiTrainRef<'_>{
    type InitializedType<'a> = Self;
    type UninitializedType = UnIntiTrainRef;
}

