use serde::{Deserialize,Serialize};
use crate::{devices::*, Position};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum MasterMessage {
    HartBeatFrom(Device),
    TrainHasReachedPosition(Train,Position),
    SendTrainTo(Train,Position)
}
