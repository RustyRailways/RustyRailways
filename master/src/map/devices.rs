use std::marker::PhantomData;
use serde::{Deserialize, Serialize};
use common_infrastructure::devices::{Switch, Train};
use crate::map::states::{MapState, MapStateInitialized};
#[derive(Debug,Serialize)]
pub struct TrainController{
    pub train: Train,
}
#[derive(Debug,Serialize)]
pub struct SwitchController{
    pub switch: Switch,
}
#[derive(Debug)]
pub enum SwitchControllerOption<'a,T: MapState<'a>>{
    NoSwitch,
    SwitchToSetStraight(T::SwitchRefType),
    SwitchToSetDiverted(T::SwitchRefType),
}
impl<'a> SwitchControllerOption<'a,MapStateInitialized> {
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





