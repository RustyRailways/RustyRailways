use std::marker::PhantomData;
use common_infrastructure::devices::{Switch, Train};
use crate::map::states::{MapState, MapStateInitialized};

pub struct TrainController<T: MapState>{
    state: PhantomData<T>,
    pub train: Train,
}

pub struct SwitchController<T: MapState>{
    state: PhantomData<T>,
    pub switch: Switch,
}

pub enum SwitchControllerOption<T: MapState>{
    NoSwitch,
    SwitchToSetStraight(T::SwitchRefType),
    SwitchToSetDiverted(T::SwitchRefType),
}
impl SwitchControllerOption<MapStateInitialized<'_>> {
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





