use std::cell::RefCell;
use std::rc::Rc;
use common_infrastructure::hals::MasterHal;
use crate::map::Map;
use crate::map::states::MapStateInitialized;

pub struct MapControllerView<'a,T:MasterHal>{
    hal: &'a T,
    map: Rc<RefCell<Map<MapStateInitialized>>>
}

impl<'a,T:MasterHal> MapControllerView<'a,T>{
    pub fn new(map: Rc<RefCell<Map<MapStateInitialized>>>, hal: &'a T) -> Self{
        MapControllerView{ map, hal }
    }
}