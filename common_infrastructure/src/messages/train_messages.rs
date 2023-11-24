use serde::{Deserialize,Serialize};

use crate::devices::Device;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum TrainMessage {
    HartBeatFrom(Device),
    SetSpeed(i8),
}