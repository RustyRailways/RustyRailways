use serde::{Deserialize,Serialize};
use crate::{devices::*, Position};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum MasterMessage {
    HeartBeatFrom(Device), // for low level controller
    TrainHasReachedPosition(Train,Position), // for low level controller
    //SendTrainTo(Train,Position) do add as rust pipe
}