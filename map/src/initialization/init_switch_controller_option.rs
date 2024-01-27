use super::CompleteInitializationMut;
use crate::devices::SwitchControllerOption;
use crate::Map;
use crate::references::IntiSwitchRef;
use crate::states::{MapStateInitialized, MapStateUninitialized};

impl CompleteInitializationMut for SwitchControllerOption<MapStateInitialized>{
    type InitFromType = SwitchControllerOption<MapStateUninitialized>;
    fn complete_initialization(& mut self, init_from: &Self::InitFromType, map: & Map<MapStateInitialized>) {
        *self = match init_from {
            SwitchControllerOption::NoSwitch => {
                SwitchControllerOption::NoSwitch
            }
            SwitchControllerOption::SwitchToSetStraight(s) => {
                SwitchControllerOption::<MapStateInitialized>::SwitchToSetStraight(
                    IntiSwitchRef{
                        switch: map.get_switch(s.switch).unwrap()
                    }
                )
            }
            SwitchControllerOption::SwitchToSetDiverted(s) => {
                SwitchControllerOption::SwitchToSetDiverted(
                    IntiSwitchRef{
                        switch: map.get_switch(s.switch).unwrap()
                    }
                )
            }
        }
    }
}