use std::cell::RefCell;
use std::rc::Rc;
use crate::map::Map;
use crate::map::states::MapStateInitialized;

pub struct MapNavigationView{
    map: Rc<RefCell<Map<MapStateInitialized>>>
}

impl MapNavigationView{
    pub fn new(map: Rc<RefCell<Map<MapStateInitialized>>>) -> Self{
        MapNavigationView{ map }
    }
}