use serde::{Deserialize,Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum TrainMessage {
    HartBeatFromMaster,
    SetSpeed(i8),
}