use serde::{Deserialize,Serialize};
use crate::{devices::*, Position};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum MasterMessage {
    TrainHasReachedPosition(Train,Position),
    RequestHeartBeat
}