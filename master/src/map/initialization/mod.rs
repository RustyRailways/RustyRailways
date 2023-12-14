use crate::map::Map;
use crate::map::states::MapStateInitialized;

pub trait CompleteInitialization<'a>{
    type InitFromType;
    fn complete_initialization(&'a self, init_from: Self::InitFromType, map: &'a Map<'a,MapStateInitialized>);
}

pub trait CompleteInitializationMut<'a>{
    type InitFromType;
    fn complete_initialization(&'a mut self, init_from: Self::InitFromType, map: &'a Map<'a,MapStateInitialized>);
}
