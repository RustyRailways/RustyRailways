use serde::{Deserialize,Serialize};
use crate::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum TrainMessage {
    HartBeat,
    SetSpeed(i8),
    SetSpeedAndStopAt(i8,Position),
}