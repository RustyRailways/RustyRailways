use anyhow::Result;
use serde::{Deserialize, Serialize};
use common_infrastructure::devices::{Switch, Train};
use common_infrastructure::hals::MasterHal;
use common_infrastructure::messages::SwitchMessage;
use common_infrastructure::Position;
use crate::map::references::{NodeRef, UnIntiNodeRef};
use crate::map::states::{MapState, MapStateInitialized, MapStateUninitialized};
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct TrainController<T: MapState>{
    pub train: Train,
    pub current_speed: i8,
    pub current_position: T::NodeRefType
}

impl TrainController<MapStateUninitialized> {
    pub fn new(train: Train) -> Self{
        TrainController{
            train,
            current_speed: 0,
            current_position: UnIntiNodeRef{
                position: Position::P1
            }
        }
    }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SwitchController{
    pub switch: Switch,
}

impl SwitchController {
    pub fn new(switch: Switch) -> Self{
        SwitchController{
            switch,
        }
    }
}
#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum SwitchControllerOption<T: MapState>{
    NoSwitch,
    SwitchToSetStraight(T::SwitchRefType),
    SwitchToSetDiverted(T::SwitchRefType),
}
impl SwitchControllerOption<MapStateInitialized> {
    pub fn set<T: MasterHal>(&self, hal: &T) -> Result<()>{
        match self {
            SwitchControllerOption::NoSwitch => Ok(()),
            SwitchControllerOption::SwitchToSetStraight(switch) => {
                hal.send_message_to_switch((**switch).switch, SwitchMessage::SetPositionStraight)
            },
            SwitchControllerOption::SwitchToSetDiverted(switch) => {
                hal.send_message_to_switch((**switch).switch, SwitchMessage::SetPositionDiverging)
            },
        }
    }
}