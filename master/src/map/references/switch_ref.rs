use crate::map::{Map,MapStateInitialized};
use crate::map::devices::SwitchController;
use crate::map::states::{ReferenceStateInitialized, ReferenceState, ReferenceStateUninitialized};
use common_infrastructure::devices::Switch;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
pub trait SwitchRef: ReferenceState {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct UnIntiSwitchRef{
    pub switch: Switch
}
impl SwitchRef for UnIntiSwitchRef{}

impl ReferenceStateUninitialized for UnIntiSwitchRef{
    fn initialize(self, map: & Map<MapStateInitialized>) -> Self::InitializedType {
        let switch = map.get_switch(self.switch).unwrap();
        IntiSwitchRef{
            switch
        }
    }

}
impl ReferenceState for UnIntiSwitchRef{
    type InitializedType = IntiSwitchRef;
    type UninitializedType = Self;
}

impl From<Switch> for UnIntiSwitchRef{
    fn from(value: Switch) -> Self {
        UnIntiSwitchRef{
            switch: value
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct IntiSwitchRef{
    pub(crate) switch: *const SwitchController
}

impl SwitchRef for IntiSwitchRef{}

impl Deref for IntiSwitchRef{
    type Target = SwitchController;
    fn deref(&self) -> &Self::Target {
        unsafe {&*self.switch}
    }
}
impl ReferenceStateInitialized for IntiSwitchRef{
    fn un_initialize(self) -> Self::UninitializedType {
        UnIntiSwitchRef{
            switch: unsafe{(*self.switch).switch}
        }
    }
}
impl ReferenceState for IntiSwitchRef{
    type InitializedType = Self;
    type UninitializedType = UnIntiSwitchRef;
}
