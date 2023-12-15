use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use common_infrastructure::devices::{Switch, Train};
use crate::map::states::{MapState, MapStateInitialized};
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct TrainController{
    pub train: Train,
}

impl TrainController {
    pub fn new(train: Train) -> Self{
        TrainController{
            train,
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
    pub fn set(&self){
        match self {
            SwitchControllerOption::NoSwitch => {},
            SwitchControllerOption::SwitchToSetStraight(switch) => {
                //switch.set_straight();
                todo!()
            },
            SwitchControllerOption::SwitchToSetDiverted(switch) => {
                //switch.set_diverted();
                todo!()
            },
        }
    }
}





