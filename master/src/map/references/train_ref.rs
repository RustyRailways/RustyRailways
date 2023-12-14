use crate::map::{Map,MapStateInitialized};
use crate::map::devices::TrainController;
use crate::map::states::{ReferenceStateInitialized, ReferenceState, ReferenceStateUninitialized};
use common_infrastructure::devices::Train;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub trait TrainRef: ReferenceState {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct UnIntiTrainRef{
    train: Train
}
impl TrainRef for UnIntiTrainRef{}

impl ReferenceStateUninitialized for UnIntiTrainRef{
    fn initialize<'a>(self, map: &'a Map<'a, MapStateInitialized>) -> Self::InitializedType<'a> {
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
    train: &'a TrainController
}

impl TrainRef for IntiTrainRef<'_>{}

impl<'a> Deref for IntiTrainRef<'a>{
    type Target = TrainController;
    fn deref(&self) -> &Self::Target {
        self.train
    }
}
impl ReferenceStateInitialized<'_> for IntiTrainRef<'_>{
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

