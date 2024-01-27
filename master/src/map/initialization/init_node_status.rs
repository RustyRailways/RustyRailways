use crate::map::initialization::UnInitialize;
use super::CompleteInitializationMut;
use crate::map::nodes::NodeStatus;
use crate::map::Map;
use crate::map::references::IntiTrainRef;
use crate::map::states::{MapStateInitialized, MapStateUninitialized, ReferenceStateInitialized};

impl CompleteInitializationMut for NodeStatus<MapStateInitialized>{
    type InitFromType = NodeStatus<MapStateUninitialized>;
    fn complete_initialization(& mut self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>) {
        *self = match init_from {
            NodeStatus::Unlocked => {
                NodeStatus::Unlocked
            }
            NodeStatus::LockedByTrain(train) => {
                NodeStatus::<MapStateInitialized>::LockedByTrain(
                    IntiTrainRef{
                        train: map.get_train(train.train).unwrap()
                    }
                )
            }
            NodeStatus::OccupiedByTrain(train) => {
                NodeStatus::<MapStateInitialized>::OccupiedByTrain(
                    IntiTrainRef{
                        train: map.get_train(train.train).unwrap()
                    }
                )
            }
        }
    }
}

impl UnInitialize for NodeStatus<MapStateInitialized> {
    type UninitializedType = NodeStatus<MapStateUninitialized>;
    fn un_initialize(self) -> Self::UninitializedType {
        match self {
            NodeStatus::Unlocked => NodeStatus::<MapStateUninitialized>::Unlocked,
            NodeStatus::LockedByTrain(t) => NodeStatus::<MapStateUninitialized>::LockedByTrain(t.un_initialize()),
            NodeStatus::OccupiedByTrain(t) => NodeStatus::<MapStateUninitialized>::OccupiedByTrain(t.un_initialize()),
        }
    }
}