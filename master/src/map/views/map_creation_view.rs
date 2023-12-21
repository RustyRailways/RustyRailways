use crate::map::Map;
use crate::map::states::MapStateUninitialized;

pub struct MapCreationView{
    map: Map<MapStateUninitialized>
}

impl MapCreationView{
    pub fn new() -> Self{
        MapCreationView{
            map: Map::new()
        }
    }

    pub fn to_map(self) -> Map<MapStateUninitialized>{
        self.map
    }
}