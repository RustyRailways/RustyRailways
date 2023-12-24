use serde::{Deserialize,Serialize};

use crate::devices::Device;
use crate::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum TrainMessage {
    HartBeatFrom(Device),
    SetSpeed(i8),
    SetSpeedAndStopAt(i8,Position),
}