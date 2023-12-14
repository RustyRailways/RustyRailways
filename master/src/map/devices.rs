use std::marker::PhantomData;
use common_infrastructure::devices::{Switch, Train};
use crate::map::states::{MapState, MapStateInitialized};

pub struct TrainController<'a,T: MapState<'a>>{
    state: PhantomData<&'a T>,
    pub train: Train,
}

pub struct SwitchController<'a,T: MapState<'a>>{
    state: PhantomData<&'a T>,
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
            },
            SwitchControllerOption::SwitchToSetDiverted(switch) => {
                //switch.set_diverted();
            },
        }
    }
}





