use std::cell::RefCell;
use std::rc::Rc;
use crate::map::Map;
use crate::map::states::MapStateInitialized;

pub struct MapControllerView{
    map: Rc<RefCell<Map<MapStateInitialized>>>
}

impl MapControllerView{
    pub fn new(map: Rc<RefCell<Map<MapStateInitialized>>>) -> Self{
        MapControllerView{ map }
    }
}