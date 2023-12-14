use std::marker::PhantomData;
use common_infrastructure::devices::{Switch, Train};
use crate::map::states::{MapState, MapStateInitialized};

pub struct TrainController{
    pub train: Train,
}

pub struct SwitchController{
    pub switch: Switch,
}

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





