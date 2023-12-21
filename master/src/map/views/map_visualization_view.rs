use std::cell::RefCell;
use std::rc::Rc;
use crate::map::Map;
use crate::map::states::MapStateInitialized;

pub struct MapVisualizationView{
    map: Rc<RefCell<Map<MapStateInitialized>>>
}

impl MapVisualizationView{
    pub fn new(map: Rc<RefCell<Map<MapStateInitialized>>>) -> Self{
        MapVisualizationView{ map }
    }
}