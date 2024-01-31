use serde::{Deserialize,Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum SwitchMessage {
    SetPositionStraight,
    SetPositionDiverted
}