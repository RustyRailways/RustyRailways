use std::cell::RefCell;
use std::ops::Deref;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use common_infrastructure::devices::{Switch, Train};
use common_infrastructure::hals::MasterHal;
use common_infrastructure::messages::SwitchMessage;
use common_infrastructure::Position;
use crate::map::map_creation_object::Direction;
use crate::map::nodes::Node;
use crate::map::references::{IntiNodeRef, UnIntiNodeRef};
use crate::map::states::{MapState, MapStateInitialized, MapStateUninitialized};
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct TrainController<T: MapState>{
    pub train: Train,
    pub current_speed: i8,
    pub direction: Direction,
    pub current_position: RefCell<T::NodeRefType>
}


impl TrainController<MapStateUninitialized> {
    pub fn new(train: Train, direction: Direction, position: Position) -> Self{
        TrainController{
            train,
            direction,
            current_speed: 0.into(),
            current_position: UnIntiNodeRef{position}.into()
        }
    }
}

impl TrainController<MapStateInitialized> {
    pub fn set_position(&self, new_position: &Node<MapStateInitialized>){
        *self.current_position.borrow_mut() = new_position.into();
    }

    pub fn get_position(&self) -> Position{
        self.current_position.borrow().position
    }

    pub fn get_current_node(&self)-> IntiNodeRef{
        self.current_position.borrow().deref().clone()
    }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SwitchController{
    pub switch: Switch,
    pub position: RefCell<SwitchPosition>
}

impl SwitchController {
    pub fn new(switch: Switch) -> Self{
        SwitchController{
            switch,
            position: SwitchPosition::Straight.into()
        }
    }
}
#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum SwitchControllerOption<T: MapState>{
    NoSwitch,
    SwitchToSetStraight(T::SwitchRefType),
    SwitchToSetDiverted(T::SwitchRefType),
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub enum SwitchPosition{
    Straight,
    Diverted
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