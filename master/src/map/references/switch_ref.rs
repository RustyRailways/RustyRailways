use crate::map::{Map,MapStateInitialized};
use crate::map::devices::SwitchController;
use crate::map::states::{InitializedState, ReferenceState, UninitializedState};
use common_infrastructure::devices::Switch;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
pub trait SwitchRef: ReferenceState {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub struct UnIntiSwitchRef{
    switch: Switch
}
impl SwitchRef for UnIntiSwitchRef{}

impl UninitializedState for UnIntiSwitchRef{
    fn initialize<'a>(self, map: &'a Map<'a, MapStateInitialized>) -> Self::InitializedType<'a> {
        let switch = map.get_switch(self.switch);
        IntiSwitchRef{
            switch
        }
    }

}
impl ReferenceState for UnIntiSwitchRef{
    type InitializedType<'a> = IntiSwitchRef<'a>;
    type UninitializedType = Self;
}

pub struct IntiSwitchRef<'a>{
    switch: &'a SwitchController<'a,MapStateInitialized>
}

impl SwitchRef for IntiSwitchRef<'_>{}

impl<'a> Deref for IntiSwitchRef<'a>{
    type Target = SwitchController<'a,MapStateInitialized>;
    fn deref(&self) -> &Self::Target {
        self.switch
    }
}
impl InitializedState<'_> for IntiSwitchRef<'_>{
    fn un_initialize(self) -> Self::UninitializedType {
        UnIntiSwitchRef{
            switch: self.switch.switch
        }
    }
}
impl ReferenceState for IntiSwitchRef<'_>{
    type InitializedType<'a> = Self;
    type UninitializedType = UnIntiSwitchRef;
}
