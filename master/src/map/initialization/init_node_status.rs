use super::CompleteInitializationMut;
use crate::map::nodes::NodeStatus;
use crate::map::Map;
use crate::map::references::IntiTrainRef;
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

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
                        train: map.get_train(train.train)
                    }
                )
            }
            NodeStatus::OccupiedByTrain(train) => {
                NodeStatus::<MapStateInitialized>::OccupiedByTrain(
                    IntiTrainRef{
                        train: map.get_train(train.train)
                    }
                )
            }
        }
    }
}