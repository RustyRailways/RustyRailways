use std::cell::RefCell;
use std::rc::Rc;
use crate::Map;
use crate::states::{MapStateInitialized, MapStateUninitialized};
use serde_json;
use crate::initialization::Initialize;
use anyhow;
use common_infrastructure::hals::MasterHal;
use crate::views::map_controller_view::MapControllerView;
use crate::views::map_creation_view::MapCreationView;
use crate::views::map_navigation_view::MapNavigationView;
use crate::views::map_visualization_view::MapVisualizationView;
use crate::map_comunication::{MapComunicationSlave,Comunicator};
pub struct MapFactory{
    pub(crate) map: Rc<RefCell<Map<MapStateInitialized>>>
}

impl MapFactory{
    pub fn new_from_map(map: Map<MapStateInitialized>) -> Self{
        MapFactory{
            map: Rc::new(RefCell::new(map))
        }
    }
    pub fn new_from_uninit_map(map: Map<MapStateUninitialized>) -> Self{
        MapFactory{
            map: Rc::new(RefCell::new(map.initialize()))
        }
    }
    pub fn new_from_json(json: &str) -> Result<Self,serde_json::Error>{
        let map = serde_json::from_str::<Map<MapStateUninitialized>>(json)?;
        let map = map.initialize();
        Ok(Self::new_from_map(map))
    }
    pub fn new_from_json_file(path: &str) -> anyhow::Result<Self>{
        let string = std::fs::read_to_string(path)?;
        Ok(Self::new_from_json(&string)?)
    }
    pub fn new_from_map_creation_view(view: MapCreationView) -> Self{
        MapFactory::new_from_uninit_map(view.to_map())
    }

    pub fn build_visualization_view(&self) -> MapVisualizationView{
        MapVisualizationView::new(self.map.clone())
    }
    pub fn build_navigation_view(&self) -> MapNavigationView{
        MapNavigationView::new(self.map.borrow())
    }
    pub fn build_controller_view<'a,T: MasterHal>(&self, hal: &'a T) -> MapControllerView<'a,T>{
        MapControllerView::new(self.map.clone(),hal)
    }
    pub fn add_comunicator(&self) -> MapComunicationSlave{
        let (master,slave) = MapComunicationSlave::get_comunicators();
        self.map.borrow_mut().add_comunicator(master);
        slave
    }
}

impl From<Map<MapStateInitialized>> for MapFactory{
    fn from(map: Map<MapStateInitialized>) -> Self {
        MapFactory::new_from_map(map)
    }
}

impl From<Map<MapStateUninitialized>> for MapFactory{
    fn from(map: Map<MapStateUninitialized>) -> Self {
        MapFactory::new_from_uninit_map(map)
    }
}

impl From<MapCreationView> for MapFactory{
    fn from(view: MapCreationView) -> Self {
        MapFactory::new_from_map_creation_view(view)
    }
}