use super::CompleteInitializationMut;
use crate::map::devices::SwitchControllerOption;
use crate::map::Map;
use crate::map::references::IntiSwitchRef;
use crate::map::states::{MapStateInitialized, MapStateUninitialized};

impl<'a> CompleteInitializationMut<'a> for SwitchControllerOption<'a,MapStateInitialized>{
    type InitFromType = SwitchControllerOption<'a,MapStateUninitialized>;
    fn complete_initialization(&'a mut self, init_from: Self::InitFromType, map: &'a Map<'a, MapStateInitialized>) {
        *self = match init_from {
            SwitchControllerOption::NoSwitch => {
                SwitchControllerOption::NoSwitch
            }
            SwitchControllerOption::SwitchToSetStraight(s) => {
                SwitchControllerOption::<'a,MapStateInitialized>::SwitchToSetStraight(
                    IntiSwitchRef{
                        switch: map.get_switch(s.switch)
                    }
                )
            }
            SwitchControllerOption::SwitchToSetDiverted(s) => {
                SwitchControllerOption::SwitchToSetDiverted(
                    IntiSwitchRef{
                        switch: map.get_switch(s.switch)
                    }
                )
            }
        }
    }
}