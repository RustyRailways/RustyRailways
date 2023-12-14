use super::CompleteInitializationMut;
use crate::map::nodes::NodeStatus;
use crate::map::Map;
use crate::map::references::IntiTrainRef;
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

impl<'a> CompleteInitializationMut<'a> for NodeStatus<'a,MapStateInitialized>{
    type InitFromType = NodeStatus<'a,MapStateUninitialized>;
    fn complete_initialization(&'a mut self, init_from: Self::InitFromType, map: &'a Map<'a, MapStateInitialized>) {
        *self = match init_from {
            NodeStatus::Unlocked => {
                NodeStatus::Unlocked
            }
            NodeStatus::LockedByTrain(train) => {
                NodeStatus::<'a,MapStateInitialized>::LockedByTrain(
                    IntiTrainRef{
                        train: map.get_train(train.train)
                    }
                )
            }
            NodeStatus::OccupiedByTrain(train) => {
                NodeStatus::<'a,MapStateInitialized>::OccupiedByTrain(
                    IntiTrainRef{
                        train: map.get_train(train.train)
                    }
                )
            }
        }
    }
}